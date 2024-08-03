
//changes made here

// models/users.rs
use diesel::prelude::*;
use diesel::Queryable;
use crate::schema::users;
use serde::{Deserialize, Serialize};

// Define the User struct which will map to the users table
#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub user_types: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub phone_number: Option<String>,
    pub bio: Option<String>,
    pub is_blocked: Option<bool>,
    pub blocked_reason: Option<String>,
    pub address: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

// Define the NewUser struct for inserting new records
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = users)]
pub struct NewUsers {
    pub user_types: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub password: String,
    pub phone_number: Option<String>,
    pub is_blocked: Option<bool>,
    pub blocked_reason: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm{
    pub email: String,
    pub password: String,
}

#[derive(Queryable,Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct RegisterForm{
    pub name: Option<String>,
    pub email:Option<String>,
    pub bio: Option<String>,
    pub password: String,
}


// use diesel::prelude::*;

// #[derive(Queryable,Selectable, Debug, Insertable)]
// #[diesel(table_name = crate::schema::users)]
// #[diesel(check_for_backend(diesel::pg::Pg))]

// pub struct Users{
//     pub id: i32, 
//     pub name: String,
//     pub email: String,
//     pub password: String,
//     pub phone_number: i32,
//     pub address: String,
//     pub updated_at: chrono::NaiveDateTime,
//     pub created_at: chrono::NaiveDateTime,

// }

// #[derive(Queryable, Selectable,Insertable)]
// #[diesel(table_name = crate::schema::users)]
// #[diesel(check_for_backend(diesel::pg::Pg))]


// pub struct NewUsers{
//     pub name: String,
//     pub email: String,
//     pub password: String,
//     pub phone_number: i32,
//     pub address: String,
// }