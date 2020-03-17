use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::schema::users;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Local};
use crate::db_connection::{PgPooledConnection};
use argon2::{self, Config};
use diesel::result::Error as DieselError;

#[derive(Debug,Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    
    pub email: String,
    pub password: String,
    pub company: String,
   pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable, Queryable, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub company: String,
    pub created_at: NaiveDateTime,
}


#[derive(Debug, Deserialize, Queryable)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub company: String,
}

impl User {
    pub fn hash(text: String) -> String {
        
        let salt = b"randomsalt";
        let config = Config::default();
        let hash = argon2::hash_encoded(text.as_bytes(), salt, &config).unwrap();
        hash
    }

    pub fn verify_hash(hased_pass: String, text_passwrod: String) -> bool {
        argon2::verify_encoded(hased_pass.as_str(), text_passwrod.as_bytes()).unwrap()
    }


    pub fn register(new_user: NewUser, conn: PgPooledConnection) -> Result<User, DieselError> {
          
        let user = diesel::insert_into(users::table).values(new_user).get_result::<User>(&conn).unwrap();
        Ok(user)
    }

    pub fn login(email: String, password: String, conn: PgPooledConnection) -> Result<User, DieselError> {
        let user = users::table.filter(users::email.eq(email)).get_result::<User>(&conn).unwrap();

       Ok(user)

       
    }

}