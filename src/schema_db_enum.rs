use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

#[derive(DbEnum, Debug, Serialize, Deserialize)]
#[DieselType = "User_role"]
pub enum UserRole {
    Guest,  // All variants must be fieldless
    User,
    Admin,
}