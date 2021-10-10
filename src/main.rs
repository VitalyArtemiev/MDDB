mod handlers;
mod schema;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
//use diesel::r2d2::{self, ConnectionManager};

use std::io::Read;
use std::path::PathBuf;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, http::StatusCode, get, post, web, dev::ServiceRequest, Error};
use std::fs::File;

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    let path: PathBuf = "/home/vitaly/dev/HomeWebPage/rundir/index.html".parse().unwrap();

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
        address = "127.0.0.1:8080"
    } else {
        address = "127.0.0.1:80"
    }

    let database_url = "postgres://medical:1-apple-day@192.168.88.235:5432/mdb";
    //let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));


    HttpServer::new(|| {
        App::new()
            .service(index)
            //.service(dynpage)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
        .bind(address)?
        .run()
        .await
}
