use custom_types::*;

table! {
    agents (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        public_key -> Varchar,
        name -> Varchar,
        organization_id -> Nullable<Varchar>,
        timestamp -> Int8,
    }
}

table! {
    users (public_key) {
        public_key -> Varchar,
        encrypted_private_key -> Varchar,
        username -> Varchar,
        hashed_password -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use super::Role;
    authorizations (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        organization_id -> Varchar,
        public_key -> Varchar,
        role -> Role,
    }
}

table! {
    blocks (block_num) {
        block_num -> Int8,
        block_id -> Varchar,
    }
}

table! {
    certificate_data (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        certificate_id -> Varchar,
        field -> Varchar,
        data -> Varchar,
    }
}

table! {
    certificates (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        certificate_id -> Varchar,
        certifying_body_id -> Varchar,
        factory_id -> Varchar,
        standard_id -> Varchar,
        standard_version -> Varchar,
        valid_from -> Int8,
        valid_to -> Int8,
    }
}

table! {
    use diesel_full_text_search::{TsVector as Tsvector};
    use diesel::sql_types::*;
    addresses (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        organization_id -> Varchar,
        street_line_1 -> Varchar,
        street_line_2 -> Nullable<Varchar>,
        city -> Varchar,
        state_province -> Nullable<Varchar>,
        country -> Varchar,
        postal_code -> Nullable<Varchar>,
        text_searchable_address_col -> Tsvector,
    }
}

table! {
    chain_record (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use super::OrganizationType;
    organizations (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        organization_id -> Varchar,
        name -> Varchar,
        organization_type -> OrganizationType,
    }
}

table! {
    contacts (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        organization_id -> Varchar,
        name -> Varchar,
        phone_number -> Varchar,
        language_code -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use super::RequestStatus;
    requests (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        request_id -> Varchar,
        factory_id -> Varchar,
        standard_id -> Varchar,
        status -> RequestStatus,
        request_date -> Int8,
    }
}

table! {
    standards (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        standard_id -> Varchar,
        organization_id -> Varchar,
        name -> Varchar,
    }
}

table! {
    standard_versions (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        standard_id -> Varchar,
        version -> Varchar,
        link -> Varchar,
        description -> Varchar,
        approval_date -> Int8,
    }
}

table! {
    accreditations (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        organization_id -> VarChar,
        standard_id -> VarChar,
        standard_version -> VarChar,
        accreditor_id -> VarChar,
        valid_from -> Int8,
        valid_to -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use super::AssertionType;
    assertions (id) {
        id -> Int8,
        start_block_num -> Int8,
        end_block_num -> Int8,
        assertion_id -> VarChar,
        assertor_pub_key -> VarChar,
        assertion_type -> AssertionType,
        object_id -> VarChar,
        data_id -> Nullable<VarChar>,
    }
}

allow_tables_to_appear_in_same_query!(
    addresses,
    agents,
    users,
    authorizations,
    blocks,
    certificate_data,
    certificates,
    chain_record,
    organizations,
    requests,
    contacts,
    standards,
    standard_versions,
    accreditations,
    assertions,
);
