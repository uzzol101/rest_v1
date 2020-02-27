#[macro_use]
extern crate diesel;

use actix_web::{web,App, HttpResponse, HttpServer, get};



mod model;
mod schema;
mod db_connection;
mod routes;





#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcom to actix")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(routes::product::init_routes)
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}