use crate::UserRole::*;
use crate::{User, UserRole};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::HeaderValue;
use actix_web::{dev::ServiceRequest, Error};
use std::collections::BTreeMap;

pub async fn extract_role(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    req.headers()
        .get("auth")
        .map(|token| vec![
            validate_token((*token.to_str().unwrap()).to_string()).to_string()
        ]
        )
        .ok_or(ErrorUnauthorized("Auth header not found"))

     //Ok(vec!["ROLE_ADMIN".to_string()])
}

//use hmac::{Hmac, NewMac};
//use jwt::claims:{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
//use sha2::Sha256;

pub fn issue_token(user: User) -> String {
    //let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");

    //let token_str = claims.sign_with_key(&key).unwrap();
    return "GUEST".to_string();
}

pub fn validate_token(token: String) -> UserRole {
    return Guest;
}


