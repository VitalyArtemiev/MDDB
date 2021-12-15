use std::collections::BTreeMap;
use crate::UserRole::*;
use crate::{User, UserRole};
use actix_web::http::HeaderValue;
use actix_web::{dev::ServiceRequest, Error};

//struct Token {}

pub async fn extract_role(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let header = req.headers().get("auth");
    return match header {
        None => Error,
        Some(token) => validate_token(*token.to_str()),
    };
}

use hmac::{Hmac, NewMac};
use jwt::claims {Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub fn issue_token(user: User) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");

    let token_str = claims.sign_with_key(&key).unwrap();
    return "GUEST".to_string();
}

pub fn validate_token(token: String) -> UserRole {
    return Guest;
}
