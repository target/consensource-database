use connection_pool::{ConnectionPool, DieselConnection};
use custom_types::OrganizationTypeEnum;
use diesel;
use diesel::prelude::*;
use errors::DatabaseError;
use models::*;
use std::i64;
use tables_schema::*;

pub const MAX_BLOCK_NUM: i64 = i64::MAX;

pub struct DataManager {
    conn: DieselConnection,
}

type ExpandedOrganization = (
    NewOrganization,
    Option<Vec<NewAccreditation>>,
    Option<NewAddress>,
    Vec<NewAuthorization>,
    Vec<NewContact>,
);

pub enum OperationType {
    CreateAgent(Vec<NewAgent>),
    CreateOrganization(Vec<ExpandedOrganization>),
    CreateCertificate(Vec<NewCertificate>),
    CreateRequest(Vec<NewRequest>),
    CreateStandard(Vec<(NewStandard, Vec<NewStandardVersion>)>),
    CreateAssertion(Vec<NewAssertion>),
    DeleteAssertion(String, i64),
}

impl DataManager {
    pub fn new(dsn: &str) -> Result<DataManager, DatabaseError> {
        let pool = ConnectionPool::connect(dsn)?;
        let conn = pool.get_connection()?;
        let manager = DataManager { conn };
        Ok(manager)
    }

    /// Submits all state changes received in a block and
    /// deals with forks and duplicates in a single db transaction.
    /// If any operations fail, all operations in the transaction will fail.
    pub fn execute_operations_in_block(
        &self,
        operations: Vec<OperationType>,
        block: &Block,
    ) -> Result<(), DatabaseError> {
        let conn = &*self.conn;
        conn.transaction::<_, _, _>(|| {
            let block_in_db = self.get_block_if_exists(block.block_num)?;
            if let Some(block_in_db) = block_in_db {
                if self.is_fork(&block_in_db, block) {
                    self.drop_fork(block.block_num)?;
                    info!(
                        "Fork detected. Replaced {} at height {}, with block {}.",
                        &block_in_db.block_id, &block_in_db.block_num, &block.block_id
                    );
                } else {
                    debug!(
                        "Block {} at height {} is a duplicate. Nothing was done.",
                        &block_in_db.block_id, &block_in_db.block_num
                    );
                    return Ok(()); // if block already exists in db and is not a fork,
                                   // it is a duplicate and nothing needs to be done
                }
            }
            for operation in operations {
                self.execute_operation(operation)?;
            }
            self.insert_block(block)?;
            debug!(
                "Successfully inserted block {} at height {} into database.",
                block.block_id, block.block_num
            );
            Ok(())
        })
    }

    fn execute_operation(&self, operation: OperationType) -> Result<(), DatabaseError> {
        match operation {
            OperationType::CreateAgent(agents) => self.insert_agent(&agents),
            OperationType::CreateOrganization(orgs_authorization) => {
                for (org, accreditations, address, authorizations, contacts) in orgs_authorization {
                    self.insert_organization(&org)?;
                    self.insert_authorizations(&authorizations, &org)?;
                    self.insert_contacts(&contacts, &org)?;
                    if org.organization_type == OrganizationTypeEnum::Factory {
                        self.insert_address(&address.unwrap(), &org)?;
                    }
                    if org.organization_type == OrganizationTypeEnum::CertifyingBody {
                        self.insert_accreditations(&accreditations.unwrap(), &org)?;
                    }
                }
                Ok(())
            }
            OperationType::CreateCertificate(certificates) => {
                self.insert_certificate(&certificates)
            }
            OperationType::CreateRequest(requests) => self.insert_request(&requests),
            OperationType::CreateStandard(standards) => {
                for (standard, versions) in standards {
                    self.insert_standard(&standard)?;
                    self.insert_standard_versions(&versions, &standard)?;
                }
                Ok(())
            }
            OperationType::CreateAssertion(assertions) => self.insert_assertions(&assertions),
            OperationType::DeleteAssertion(address, current_block_num) => {
                self.delete_assertion(&address, current_block_num)
            }
        }
    }

    fn insert_block(&self, block: &Block) -> Result<(), DatabaseError> {
        diesel::insert_into(blocks::table)
            .values(block)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn get_block_if_exists(&self, block_num: i64) -> Result<Option<Block>, DatabaseError> {
        let mut blocks = blocks::table.find(block_num).load::<Block>(&*self.conn)?;
        if blocks.is_empty() {
            return Ok(None);
        }
        Ok(Some(blocks.remove(0)))
    }

    fn is_fork(&self, block_in_db: &Block, block_to_be_inserted: &Block) -> bool {
        if block_in_db.block_id == block_to_be_inserted.block_id {
            return false;
        }
        true
    }

    fn insert_agent(&self, agents: &[NewAgent]) -> Result<(), DatabaseError> {
        for agent in agents {
            self.update_agent(&agent.public_key, agent.start_block_num)?;
        }
        diesel::insert_into(agents::table)
            .values(agents)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_agent(
        &self,
        agent_public_key: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_agents_query = agents::table
            .filter(agents::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(agents::public_key.eq(agent_public_key));
        diesel::update(modified_agents_query)
            .set(agents::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_organization(&self, org: &NewOrganization) -> Result<(), DatabaseError> {
        self.update_organization(&org.organization_id, org.start_block_num)?;
        diesel::insert_into(organizations::table)
            .values(org)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_organization(
        &self,
        organization_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_org_query = organizations::table
            .filter(organizations::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(organizations::organization_id.eq(organization_id));
        diesel::update(modified_org_query)
            .set(organizations::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_address(
        &self,
        address: &NewAddress,
        org: &NewOrganization,
    ) -> Result<(), DatabaseError> {
        self.update_address(&org.organization_id, org.start_block_num)?;
        diesel::insert_into(addresses::table)
            .values(address)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_address(
        &self,
        organization_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_address_query = addresses::table
            .filter(addresses::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(addresses::organization_id.eq(organization_id));
        diesel::update(modified_address_query)
            .set(addresses::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_authorizations(
        &self,
        auths: &[NewAuthorization],
        org: &NewOrganization,
    ) -> Result<(), DatabaseError> {
        self.update_authorizations(&org.organization_id, org.start_block_num)?;
        diesel::insert_into(authorizations::table)
            .values(auths)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_authorizations(
        &self,
        organization_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_auth_query = authorizations::table
            .filter(authorizations::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(authorizations::organization_id.eq(organization_id));
        diesel::update(modified_auth_query)
            .set(authorizations::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_contacts(
        &self,
        contacts: &[NewContact],
        org: &NewOrganization,
    ) -> Result<(), DatabaseError> {
        self.update_contacts(&org.organization_id, org.start_block_num)?;
        diesel::insert_into(contacts::table)
            .values(contacts)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_contacts(
        &self,
        organization_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_contacts_query = contacts::table
            .filter(contacts::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(contacts::organization_id.eq(organization_id));
        diesel::update(modified_contacts_query)
            .set(contacts::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_certificate(&self, certs: &[NewCertificate]) -> Result<(), DatabaseError> {
        for cert in certs {
            self.update_certificate(&cert.certificate_id, cert.start_block_num)?;
        }
        diesel::insert_into(certificates::table)
            .values(certs)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_certificate(
        &self,
        certificate_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_cert_query = certificates::table
            .filter(certificates::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(certificates::certificate_id.eq(certificate_id));
        diesel::update(modified_cert_query)
            .set(certificates::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_request(&self, requests: &[NewRequest]) -> Result<(), DatabaseError> {
        for request in requests {
            self.update_request(&request.request_id, request.start_block_num)?;
        }
        diesel::insert_into(requests::table)
            .values(requests)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_request(
        &self,
        request_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_req_query = requests::table
            .filter(requests::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(requests::request_id.eq(request_id));
        diesel::update(modified_req_query)
            .set(requests::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_standard(&self, standard: &NewStandard) -> Result<(), DatabaseError> {
        self.update_standard(&standard.standard_id, standard.start_block_num)?;
        diesel::insert_into(standards::table)
            .values(standard)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_standard(
        &self,
        standard_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_standard_query = standards::table
            .filter(standards::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(standards::standard_id.eq(standard_id));
        diesel::update(modified_standard_query)
            .set(standards::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_standard_versions(
        &self,
        versions: &[NewStandardVersion],
        standard: &NewStandard,
    ) -> Result<(), DatabaseError> {
        self.update_standard_versions(&standard.standard_id, standard.start_block_num)?;
        diesel::insert_into(standard_versions::table)
            .values(versions)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_standard_versions(
        &self,
        standard_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_versions_query = standard_versions::table
            .filter(standard_versions::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(standard_versions::standard_id.eq(standard_id));
        diesel::update(modified_versions_query)
            .set(standard_versions::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_accreditations(
        &self,
        accreditations: &[NewAccreditation],
        org: &NewOrganization,
    ) -> Result<(), DatabaseError> {
        self.update_accreditations(&org.organization_id, org.start_block_num)?;
        diesel::insert_into(accreditations::table)
            .values(accreditations)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_accreditations(
        &self,
        organization_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_accreditations_query = accreditations::table
            .filter(accreditations::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(accreditations::organization_id.eq(organization_id));
        diesel::update(modified_accreditations_query)
            .set(accreditations::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn insert_assertions(&self, assertions: &[NewAssertion]) -> Result<(), DatabaseError> {
        for assertion in assertions {
            self.update_assertion(&assertion.assertion_id, assertion.start_block_num)?;
        }
        diesel::insert_into(assertions::table)
            .values(assertions)
            .execute(&*self.conn)?;
        Ok(())
    }

    fn update_assertion(
        &self,
        assertion_id: &str,
        current_block_num: i64,
    ) -> Result<(), DatabaseError> {
        let modified_assertions_query = assertions::table
            .filter(assertions::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(assertions::assertion_id.eq(assertion_id));
        diesel::update(modified_assertions_query)
            .set(assertions::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn delete_assertion(&self, address: &str, current_block_num: i64) -> Result<(), DatabaseError> {
        let assertions_query = assertions::table
            .filter(assertions::end_block_num.eq(MAX_BLOCK_NUM))
            .filter(assertions::address.eq(address));
        diesel::update(assertions_query)
            .set(assertions::end_block_num.eq(current_block_num))
            .execute(&*self.conn)?;
        Ok(())
    }

    fn drop_fork(&self, block_num: i64) -> Result<(), DatabaseError> {
        let to_drop_query = chain_record::table.filter(chain_record::start_block_num.ge(block_num));
        diesel::delete(to_drop_query).execute(&*self.conn)?;
        let to_update_query = chain_record::table.filter(chain_record::end_block_num.ge(block_num));
        diesel::update(to_update_query)
            .set(chain_record::end_block_num.eq(MAX_BLOCK_NUM))
            .execute(&*self.conn)?;
        let blocks_to_drop_query = blocks::table.filter(blocks::block_num.ge(block_num));
        diesel::delete(blocks_to_drop_query).execute(&*self.conn)?;
        Ok(())
    }

    pub fn fetch_known_blocks(&self) -> Result<Vec<Block>, DatabaseError> {
        blocks::table
            .order(blocks::block_num.desc())
            .load::<Block>(&*self.conn)
            .map_err(DatabaseError::TransactionError)
    }
}
