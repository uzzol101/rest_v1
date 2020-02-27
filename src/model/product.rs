use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::products;
use crate::db_connection::establish_connection;


#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32> // For a value that can be null,                       
}

#[derive(Insertable, Deserialize)]
#[table_name="products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}


impl Product {
    pub fn all_product() -> Vec<Product> {
        let conn = establish_connection();

        let resutl = products::table.load::<Product>(&conn).unwrap();
        resutl
    }

    pub fn create_product(new_product: NewProduct) -> Result<Self, diesel::result::Error> {
        let conn = establish_connection();
     
        let result = diesel::insert_into(products::table).values(new_product).get_result(&conn).unwrap();

        Ok(result)
    }

    pub fn findById(id: i32) -> Product {
        let conn = establish_connection();
        let result = products::table.find(id).first::<Product>(&conn).unwrap();

        result
    }

    pub fn findByIdAndUpdate(product: Product) -> Product {
        let conn = establish_connection();
        let result = diesel::update(products::table).filter(products::id.eq(product.id)).set(product).get_result::<Product>(&conn).unwrap();
        
        //let result = diesel::update(products::table.find(product.id)).set(product).get_result::<Product>(&conn).unwrap();

        result
    }

    pub fn findByIdAndRemove(id: i32) -> usize {
        let conn = establish_connection();
        let result = diesel::delete(products::table.find(id)).execute(&conn).unwrap();

        result
    }
}