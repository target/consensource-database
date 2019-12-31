use custom_types::*;
use tables_schema::*;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_num: i64,
    pub block_id: String,
}

#[derive(Queryable, Serialize)]
pub struct Agent {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub public_key: String,
    pub name: String,
    pub organization_id: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "agents"]
pub struct NewAgent {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub public_key: String,
    pub name: String,
    pub organization_id: Option<String>,
    pub timestamp: i64,
}

#[derive(Queryable, Serialize)]
pub struct Authorization {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub public_key: String,
    pub role: RoleEnum,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "authorizations"]
pub struct NewAuthorization {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub public_key: String,
    pub role: RoleEnum,
}

#[derive(Queryable, Insertable, Default, Clone)]
#[table_name = "addresses"]
pub struct Address {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub street_line_1: String,
    pub street_line_2: Option<String>,
    pub city: String,
    pub state_province: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "addresses"]
pub struct NewAddress {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub street_line_1: String,
    pub street_line_2: Option<String>,
    pub city: String,
    pub state_province: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
}

#[derive(Clone, Queryable, Serialize)]
pub struct Organization {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub name: String,
    pub organization_type: OrganizationTypeEnum,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub name: String,
    pub organization_type: OrganizationTypeEnum,
}

#[derive(Queryable, Serialize)]
pub struct Contact {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub name: String,
    pub phone_number: String,
    pub language_code: String,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "contacts"]
pub struct NewContact {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub name: String,
    pub phone_number: String,
    pub language_code: String,
}

#[derive(Queryable, Serialize, Default)]
pub struct Certificate {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub certificate_id: String,
    pub certifying_body_id: String,
    pub factory_id: String,
    pub standard_id: String,
    pub standard_version: String,
    pub valid_from: i64,
    pub valid_to: i64,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "certificates"]
pub struct NewCertificate {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub certificate_id: String,
    pub certifying_body_id: String,
    pub factory_id: String,
    pub standard_id: String,
    pub standard_version: String,
    pub valid_from: i64,
    pub valid_to: i64,
}

#[derive(Queryable, Serialize)]
pub struct CertificateData {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub certificate_id: String,
    pub field: String,
    pub data: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "certificate_data"]
pub struct NewCertificateData {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub certificate_id: String,
    pub field: String,
    pub data: String,
}

#[derive(Queryable, Insertable, Clone)]
pub struct User {
    pub public_key: String,
    pub transaction_id: String,
    pub batch_id: String,
    pub encrypted_private_key: String,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Request {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub request_id: String,
    pub factory_id: String,
    pub standard_id: String,
    pub status: RequestStatusEnum,
    pub request_date: i64,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "requests"]
pub struct NewRequest {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub request_id: String,
    pub factory_id: String,
    pub standard_id: String,
    pub status: RequestStatusEnum,
    pub request_date: i64,
}

#[derive(Queryable, Serialize)]
pub struct Standard {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub standard_id: String,
    pub organization_id: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "standards"]
pub struct NewStandard {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub standard_id: String,
    pub organization_id: String,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct StandardVersion {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub standard_id: String,
    pub version: String,
    pub link: String,
    pub description: String,
    pub approval_date: i64,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "standard_versions"]
pub struct NewStandardVersion {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub standard_id: String,
    pub version: String,
    pub link: String,
    pub description: String,
    pub approval_date: i64,
}

#[derive(Queryable, Serialize)]
pub struct RetailerFactories {
    pub id: i64,
    pub factory_id: String,
    pub factory_name: String,
    pub contact_name: String,
    pub contact_phone_number: String,
    pub contact_language_code: String,
    pub country: String,
    pub state_province: String,
    pub city: String,
    pub street_line_1: String,
    pub street_line_2: String,
}

#[derive(Queryable, Serialize)]
pub struct Accreditation {
    pub id: i64,
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub standard_id: String,
    pub standard_version: String,
    pub accreditor_id: String,
    pub valid_from: i64,
    pub valid_to: i64,
}

#[derive(Debug, PartialEq, Queryable, Insertable)]
#[table_name = "accreditations"]
pub struct NewAccreditation {
    pub start_block_num: i64,
    pub end_block_num: i64,
    pub organization_id: String,
    pub standard_id: String,
    pub standard_version: String,
    pub accreditor_id: String,
    pub valid_from: i64,
    pub valid_to: i64,
}
