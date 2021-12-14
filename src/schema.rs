// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole{
        Guest,
        User,
        Admin
    };
}

diesel::table! {
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

diesel::table! {
    medical_institutions (id) {
        id -> Int4,
        fullname -> Varchar,
        address -> Nullable<Varchar>,
    }
}

diesel::table! {
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (key) {
        key -> Int4,
        login -> Varchar,
        role -> UserRole,
        salt -> Varchar,
        hash -> Varchar,
        date_pass_changed -> Date,
    }
}

diesel::joinable!(documents -> medical_institutions (institution_id));
diesel::joinable!(documents -> patients (patient_id));

diesel::allow_tables_to_appear_in_same_query!(
    documents,
    medical_institutions,
    patients,
    users,
);
