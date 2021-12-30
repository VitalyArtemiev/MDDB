use std::fmt::{Display, Formatter};
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

#[derive(DbEnum, Debug, Serialize, Deserialize)]
#[DieselType = "User_role"]
pub enum UserRole {
    Guest,  // All variants must be fieldless
    User,
    Admin,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Guest => f.write_str("ROLE_GUEST")?,
            UserRole::User => f.write_str("ROLE_USER")?,
            UserRole::Admin => f.write_str("ROLE_ADMIN")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(UserRole::Guest.to_string(), "ROLE_GUEST");
    }
}

