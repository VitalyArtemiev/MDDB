use actix_web::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub async fn get_users(db: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    format!("hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}