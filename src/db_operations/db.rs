use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub fn establish_connection() -> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in path");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to db"))
}




// mod models;
// mod schema;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;
// use crate::models::users::{NewUser, User};

// fn main() {
//     use schema::users::dsl::*;

//     // Load environment variables from .env file
//     dotenv().ok();

//     // Retrieve the database URL from the environment
//     let database_url = env::var("DATABASE_URL").expect("Database URL cannot be found");

//     // Establish a connection to the PostgreSQL database
//     let mut connection = PgConnection::establish(&database_url)
//         .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e));

//     // Define a new user instance  
//     let example = NewUser {
//         // id: 1,  
//         user_type: None,
//         name: Some("Wasonga".to_string()),
//         email: Some("erick@gmail.com".to_string()),
//         bio: Some("I love code crafting".to_string()),
//         is_blocked: None,
//         blocked_reason: None,
//         password: "wasonga".to_string(),
//     };

//     // Insert the new user into the database and retrieve the inserted record
//     let database_record: User = diesel::insert_into(schema::users::table)
//         .values(&example)
//         .get_result(&mut connection)
//         .expect("Error saving user");

//     // Print the inserted record
//     println!("Inserted {:?}", database_record);

//     /* start of select */
//         // let results = users.select(users::as_select()).load(connection).expect("unable to select");
//         // println!("Displaying {:#?}",results);
//     /*end of select */
        
//     /* start of select filter*/    
//         // let results = users.filter(id.eq(2)).select(users::as_select()).load(connection).expect("Unable to select");
//         // println!("Displaying {:#?}",results);
//     /* start of select filter*/

//     //*start of delete*/
//         // let num_delete = diesel::delete(users.filter(id.eq(6)))
//         // .execute(&mut connection)
//         // .expect("Error Deleting Posts");

//         // println!("Deleted {} posts ",num_delete);
//     /*end of delete*/


    
// }