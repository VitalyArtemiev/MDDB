table! {
    documents (id) {
        id -> Int4,
        patient_id -> Int4,
        institution_id -> Nullable<Int4>,
        date_a -> Nullable<Date>,
        date_b -> Nullable<Date>,
        date_c -> Nullable<Date>,
        date_d -> Nullable<Date>,
        diagnosis -> Nullable<Text>,
        anamnesis -> Nullable<Text>,
        recommendations -> Nullable<Text>,
        document_date -> Nullable<Date>,
    }
}

table! {
    medical_institutions (id) {
        id -> Int4,
        fullname -> Varchar,
        address -> Nullable<Varchar>,
    }
}

table! {
    patients (id) {
        id -> Int4,
        first_name -> Varchar,
        second_name -> Nullable<Varchar>,
        last_name -> Varchar,
        dob -> Date,
        address -> Varchar,
        diagnosis -> Nullable<Varchar>,
        diagnosis_code -> Nullable<Varchar>,
        occupation -> Nullable<Varchar>,
    }
}

table! {
    use diesel::types::*;
    use crate::schema_db_enum::User_roleMapping;
    users (key) {
        key -> Int4,
        login -> Varchar,
        role -> User_roleMapping,
        salt -> Varchar,
        hash -> Varchar,
    }
}

joinable!(documents -> medical_institutions (institution_id));
joinable!(documents -> patients (patient_id));

allow_tables_to_appear_in_same_query!(
    documents,
    medical_institutions,
    patients,
    users,
);
