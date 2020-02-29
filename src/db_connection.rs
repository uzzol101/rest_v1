use diesel::prelude::*;
use diesel::pg::PgConnection;
use actix_web::{web, HttpResponse, get};
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager, PoolError};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


fn init_pool(db_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}


pub fn establish_connection() -> PgPool {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").expect("database url not set");
    PgConnection::establish(&db_url).expect(&format!("unable to establish connection {}", db_url));

    init_pool(&db_url).expect("unable to create pool")
}

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
       .get()
       .map_err(|e| {
       HttpResponse::InternalServerError().json(e.to_string())
       })
}