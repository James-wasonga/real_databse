// use crate::models::loans;
// use diesel::prelude::*;
// use diesel::Queryable;
// use crate::schema::users;

// #[derive(Queryable,Selectable, Insertable)]
// #[diesel(table_name = loans)]
// #[derive(Debug)]

// pub struct Loans{
//     pub id: Option<i32>,
//     pub user_id: Option<i32>,
//     pub amount: Option<i32>,
//     pub interest_rate: Option<i32>,
//     pub status: Option<bool>,
//     pub updated_at: chrono::NaiveDateTime ,
//     pub created_at: chrono::NaiveDateTime
// }

// #[derive(Queryable,Insertable,Selectable)]
// #[diesel(table_name = loans)]


// pub struct NewLoans{
//     pub amount: Option<i32>,
//     pub interest_rate: Option<i32>,
//     pub status: Option<bool>,
// }