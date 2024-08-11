mod models;
mod controllers;
mod db_operations;
mod schema;


use models::app_state;
use controllers::users::{login_page,register_page,login_user, register_user};
use db_operations::db::establish_connection;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenvy::dotenv;
use crate::models::app_state::AppState;
use std::env;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::controllers::home::default_handler;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
   dotenv().ok();

//    let host = env::var("HOST").expect("Host not found");
//    let port = env::var("PORT").expect("Port not found");
//    let host_port = format!("{}:{}",host,port);

   HttpServer::new(move || {
    
        let app_state = web::Data::new(AppState{
            db_connection:Mutex::new(establish_connection())
        });

        App::new().app_data(app_state.clone())
        .route("/login", web::get().to(login_page))
        .route("/login", web::post().to(login_user))
        .route("/register", web::get().to(register_page))
        .route("/register", web::post().to(register_user))
        .default_service(web::to(default_handler))

   })
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}

// use std::{convert::Infallible, io};
// use std::os::linux::raw::stat;
// use std::sync::Mutex;
// use actix_web::web::get;
// use diesel::pg::sql_types::Uuid;
// use actix_session::Session;

 
// use actix_files::{Files, NamedFile};
// use actix_session::{storage::CookieSessionStore, SessionMiddleware};
// use actix_web::{
//     error, get,
//     http::{
//         header::{self, ContentType},
//         Method, StatusCode,
//     },
//     middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
// };
// use actix_web_lab::extract::Path;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct User {
//     // id: i32,
//     // user_type: Option<i32>,
//     // name: Option<String>,
//     // email: Option<String>,
//     // bio: Option<String>,
//     // is_blocked: Option<bool>,
//     // blocked_reason: Option<String>,
//     // password: String,
//     id: i32,
//     user_type: i32,
//     name: String,
//     email: String,
//     bio: String,
//     is_blocked: bool,
//     blocked_reason: String,
//     password: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Loan {
//     // id: String, // Changed to String for UUID
//     user_id: i32,
//     amount: f64,
//     interest_rate: f64,
//     duration: usize,
//     status: String,
// }

// #[derive(Debug)]
// struct AppState {
//     users: Mutex<Vec<User>>,
//     loans: Mutex<Vec<Loan>>,
// }

// async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
//     let all_persons = state.users.lock().unwrap();
//     HttpResponse::Ok().json(&*all_persons)
// }

// async fn add_users(user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
//     let mut all_persons = state.users.lock().unwrap();
//     all_persons.push(user.into_inner());
//     println!("Persons are {}", all_persons.len());
//     HttpResponse::Ok().finish()
// }

// async fn user_login(user: web::Json<User>, session: Session, state: web::Data<AppState>) -> impl Responder {
//     let users = state.users.lock().unwrap();
//     for stored_user in users.iter() {
//         if stored_user.email == user.email && stored_user.password == user.password {
//             session.insert("user_id", stored_user.id).unwrap();
//             return HttpResponse::Ok().json(stored_user);
//         }
//     }
//     HttpResponse::Unauthorized().body("Invalid credentials")
// }

// async fn user_logout(session: Session) -> impl Responder {
//     session.clear();
//     HttpResponse::Ok().body("Logged out")
// }

// // async fn add_loan(loan: web::Json<Loan>, state: web::Data<AppState>, session: Session) -> impl Responder {
// //     if let Some(user_id) = session.get::<usize>("user_id").unwrap() {
// //         let mut loans = state.loans.lock().unwrap();
// //         let mut new_loan = loan.into_inner();
// //         new_loan.user_id = user_id;
// //         new_loan.id = Uuid::new_v4().to_string(); // Generate UUID
// //         loans.push(new_loan);
// //         HttpResponse::Ok().finish()
// //     } else {
// //         HttpResponse::Unauthorized().body("You are not logged in")
// //     }
// // }
// async fn add_loan(loan: web::Json<Loan>, state: web::Data<AppState>) ->impl Responder{
//     let mut loans  = state.loans.lock().unwrap();
//     let users = state.users.lock().unwrap();

//     if users.iter().any(|user| user.id == loan.user_id){
//         loans.push(loan.into_inner());
//         HttpResponse::Ok().finish()
//     }else{
//         HttpResponse::NotFound().body("User doesn't exist")

//     }
// }

// async fn get_user_loans(state: web::Data<AppState>) -> impl Responder{
//     HttpResponse::Ok().json(&state.loans)
// }
// // async fn get_user_loans(state: web::Data<AppState>, session: Session) -> impl Responder {
// //     if let Some(user_id) = session.get::<usize>("user_id").unwrap() {
// //         let loans = state.loans.lock().unwrap();
// //         let user_loans: Vec<Loan> = loans.iter().filter(|&loan| loan.user_id == user_id).cloned().collect();
// //         HttpResponse::Ok().json(user_loans)
// //     } else {
// //         HttpResponse::Unauthorized().body("Not logged in")
// //     }
// // }

// async fn post_example() -> impl Responder{
//     HttpResponse::Ok().body("post is working")
// }


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let app_state =  web::Data::new(AppState{users: Mutex::new(Vec::new()), loans: Mutex::new(Vec::new())});
//     HttpServer::new(move|| {
//       App::new()
//             .app_data(app_state.clone())
//             .route("/get_users", web::get().to(get_all_users))
//             .route("/add_users", web::post().to(add_users))
//             .route("/login", web::post().to(user_login))
//             .route("/logout", web::post().to(user_logout))
//             .route("/add_loan", web::post().to(add_loan))
//             .route("/get_user_loans", web::get().to(get_user_loans))
//             .route("/post_example", web::post().to(post_example))
//             // .default_service(web::to(default_hnadler))

//             })
//         .bind(("127.0.0.1", 8080))? 
//         .run()  
//         .await

// }




