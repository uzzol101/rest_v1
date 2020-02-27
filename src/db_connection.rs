use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("database url not set");
    PgConnection::establish(&db_url).expect(&format!("unable to establish connection {}", db_url))
}
