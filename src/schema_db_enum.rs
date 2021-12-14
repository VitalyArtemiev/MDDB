use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

#[derive(DbEnum, Debug, Serialize, Deserialize)]
pub enum User_role {
    Guest,  // All variants must be fieldless
    User,
    Admin,
}