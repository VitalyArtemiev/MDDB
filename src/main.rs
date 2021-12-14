#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::handlers::InputUser;
use crate::schema_db_enum::UserRole;
use crate::schema_db_enum::UserRole::Admin;
use actix_web::{
    dev::ServiceRequest, get, http::StatusCode, post, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder, Result,
};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::schema::users::dsl::users;

use self::models::*;

mod handlers;
mod models;
mod schema;
mod schema_db_enum;

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    let path: PathBuf = "/home/vitaly/dev/HomeWebPage/rundir/index.html"
        .parse()
        .unwrap();

    let mut file = File::open("/home/vitaly/dev/HomeWebPage/rundir/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(contents))

    //.body(include_str!("/home/vitaly/dev/HomeWebPage/rundir/index.html")))
}

/*#[get("/test")]
async fn dynpage(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "/home/vitaly/dev/HomeWebPage/rundir/index.html".parse().unwrap();
    Ok(NamedFile::open(path)? as str)
}*/

// #[get("/login")]
// async fn login() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body())
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address;
    if cfg!(debug_assertions) {
        address = "127.0.0.1:8080";
    } else {
        address = "127.0.0.1:80"
    }

    dotenv().ok();

    //let database_url = "postgres://medical:1-apple-day@192.168.88.235:5432/mdb";
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool

    let pool =
        r2d2::Pool::new(ConnectionManager::<PgConnection>::new(&database_url)).expect(&format!(
            "DB connection pool creation failed, DB URL: {}",
            database_url
        ));

    if env::var("GEN_ADMIN").is_ok() {
        println!("Due to env flag, new admin will be generated if none are present");

        handlers::add_user(
            pool.get().unwrap(),
            "admin".to_string(),
            UserRole::Admin,
            env::var("GEN_ADMIN").unwrap(),
        )
        .map(|_| println!("Default admin created"))
        .map_err(|it|println!("Failed to create new admin: {}", it));
    }

    HttpServer::new(|| {
        App::new()
            .service(index)
            //.service(dynpage)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/users", web::get().to(handlers::handle_get_users))
            .route(
                "/users/{id}",
                web::get().to(handlers::handle_get_user_by_id),
            )
            .route("/users", web::post().to(handlers::handle_add_user))
            .route(
                "/users/{id}",
                web::delete().to(handlers::handle_delete_user),
            )
    })
    .bind(address)?
    .run()
    .await
}
