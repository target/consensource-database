use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

// Role

#[derive(SqlType)]
#[postgres(type_name = "role")]
pub struct Role;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Clone)]
#[sql_type = "Role"]
pub enum RoleEnum {
    Admin,
    Transactor,
    UnsetRole,
}

impl ToSql<Role, Pg> for RoleEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            RoleEnum::Admin => out.write_all(b"ADMIN")?,
            RoleEnum::Transactor => out.write_all(b"TRANSACTOR")?,
            RoleEnum::UnsetRole => out.write_all(b"UNSET_ROLE")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Role, Pg> for RoleEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"ADMIN" => Ok(RoleEnum::Admin),
            b"TRANSACTOR" => Ok(RoleEnum::Transactor),
            b"UNSET_ROLE" => Ok(RoleEnum::UnsetRole),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// OrganizationType

#[derive(SqlType, QueryId)]
#[postgres(type_name = "organization_type")]
pub struct OrganizationType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize, Clone)]
#[sql_type = "OrganizationType"]
pub enum OrganizationTypeEnum {
    StandardsBody,
    CertifyingBody,
    Factory,
    Ingestion,
    UnsetType,
}

impl ToSql<OrganizationType, Pg> for OrganizationTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            OrganizationTypeEnum::CertifyingBody => out.write_all(b"CERTIFYING_BODY")?,
            OrganizationTypeEnum::StandardsBody => out.write_all(b"STANDARDS_BODY")?,
            OrganizationTypeEnum::Factory => out.write_all(b"FACTORY")?,
            OrganizationTypeEnum::Ingestion => out.write_all(b"INGESTION")?,
            OrganizationTypeEnum::UnsetType => out.write_all(b"UNSET_TYPE")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OrganizationType, Pg> for OrganizationTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"CERTIFYING_BODY" => Ok(OrganizationTypeEnum::CertifyingBody),
            b"STANDARDS_BODY" => Ok(OrganizationTypeEnum::StandardsBody),
            b"FACTORY" => Ok(OrganizationTypeEnum::Factory),
            b"INGESTION" => Ok(OrganizationTypeEnum::Ingestion),
            b"UNSET" => Ok(OrganizationTypeEnum::UnsetType),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// RequestStatus

#[derive(SqlType)]
#[postgres(type_name = "status")]
pub struct RequestStatus;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize)]
#[sql_type = "RequestStatus"]
pub enum RequestStatusEnum {
    Open,
    InProgress,
    Closed,
    Certified,
    UnsetStatus,
    PreCertified,
}

impl ToSql<RequestStatus, Pg> for RequestStatusEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            RequestStatusEnum::Open => out.write_all(b"OPEN")?,
            RequestStatusEnum::InProgress => out.write_all(b"IN_PROGRESS")?,
            RequestStatusEnum::Closed => out.write_all(b"CLOSED")?,
            RequestStatusEnum::Certified => out.write_all(b"CERTIFIED")?,
            RequestStatusEnum::UnsetStatus => out.write_all(b"UNSET_STATUS")?,
            RequestStatusEnum::PreCertified => out.write_all(b"PRE_CERTIFIED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<RequestStatus, Pg> for RequestStatusEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"OPEN" => Ok(RequestStatusEnum::Open),
            b"IN_PROGRESS" => Ok(RequestStatusEnum::InProgress),
            b"CLOSED" => Ok(RequestStatusEnum::Closed),
            b"CERTIFIED" => Ok(RequestStatusEnum::Certified),
            b"UNSET_STATUS" => Ok(RequestStatusEnum::UnsetStatus),
            b"PRE_CERTIFIED" => Ok(RequestStatusEnum::PreCertified),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// AssertionType

#[derive(SqlType, QueryId)]
#[postgres(type_name = "assertion_type")]
pub struct AssertionType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize, Clone)]
#[sql_type = "AssertionType"]
pub enum AssertionTypeEnum {
    Standard,
    Certificate,
    Factory,
    UnsetType,
}

impl ToSql<AssertionType, Pg> for AssertionTypeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            AssertionTypeEnum::Standard => out.write_all(b"STANDARD")?,
            AssertionTypeEnum::Certificate => out.write_all(b"CERTIFICATE")?,
            AssertionTypeEnum::Factory => out.write_all(b"FACTORY")?,
            AssertionTypeEnum::UnsetType => out.write_all(b"UNSET_TYPE")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<AssertionType, Pg> for AssertionTypeEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"STANDARD" => Ok(AssertionTypeEnum::Standard),
            b"CERTIFICATE" => Ok(AssertionTypeEnum::Certificate),
            b"FACTORY" => Ok(AssertionTypeEnum::Factory),
            b"UNSET" => Ok(AssertionTypeEnum::UnsetType),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
