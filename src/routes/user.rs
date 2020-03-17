use actix_web::{web,HttpResponse, HttpRequest, get, post, put, delete};
use crate::model::user::{User,NewUser, RegisterUser, AuthUser};
use serde_json::json;
use chrono::{NaiveDateTime, Local};
use crate::db_connection::{PgPool, pg_pool_handler};

#[post("/register")]
async fn register(new_user: web::Json<RegisterUser>, pool: web::Data<PgPool>) -> HttpResponse {
    println!("here is user");
    let pg_pool = pg_pool_handler(pool).unwrap();
  
        let u = NewUser{
            email: new_user.email.clone(),
            password: User::hash(new_user.password.clone()),
            company: new_user.company.clone(),
            created_at: Local::now().naive_local(),
        };
    
    let user = User::register(u, pg_pool).unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/login")]
async fn login(auth_user: web::Json<AuthUser>, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = pg_pool_handler(pool).unwrap();

    let user = User::login(auth_user.email.clone(), auth_user.password.clone(), pg_pool).unwrap();
    if User::verify_hash(user.password.clone(), auth_user.password.clone()) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Ok().json(json!({"user": "not found"}))
    }

}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}