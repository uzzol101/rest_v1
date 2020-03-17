use actix_web::{web,HttpResponse, HttpRequest, get, post, put, delete};
use crate::model::product::{Product,NewProduct};
use serde_json::json;
use crate::db_connection::{PgPool, pg_pool_handler};

#[get("/products")]
async fn product_list(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = pg_pool_handler(pool).unwrap();
    HttpResponse::Ok().json(Product::all_product(pg_pool))
}

#[get("/product/{id}")]
async fn find_product(_req: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = pg_pool_handler(pool).unwrap();
    let result = Product::findById(id.into_inner(), pg_pool);
    HttpResponse::Ok().json(result)
}

#[post("/products")]
async fn create_product(_req: HttpRequest, product: web::Json<NewProduct>, pool: web::Data<PgPool>) -> HttpResponse {
    println!("saving products");
    let pg_pool = pg_pool_handler(pool).unwrap();
    let result = Product::create_product(product.into_inner(), pg_pool).unwrap();
    HttpResponse::Ok().json(result)
}

#[put("/product/{id}")]
async fn update_product(_req: HttpRequest, product: web::Json<Product>, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = pg_pool_handler(pool).unwrap();
    let result = Product::findByIdAndUpdate(product.into_inner(), pg_pool);
    HttpResponse::Ok().json(result)
}
#[delete("/product/{id}")]
async fn destroy_product(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = pg_pool_handler(pool).unwrap();
    let result = Product::findByIdAndRemove(id.into_inner(), pg_pool);
    HttpResponse::Ok().json(json!({"Ok": result}))
}

pub fn init_routes(cfg:  &mut web::ServiceConfig) {
    cfg.service(product_list);
    cfg.service(find_product);
    cfg.service(create_product);
    cfg.service(update_product);
    cfg.service(destroy_product);
}