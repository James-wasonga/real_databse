use askama::Template;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;


#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate{
    pub(crate) error: Option<String>,
    pub(crate) message: Option<String>
}

#[derive(Template)]
#[template(path="register.html")]
pub struct RegisterTemplate{
    pub(crate) error: Option<String>
}


// #[derive(Template)]
// #[template(path="dashboard.html")]
// pub struct DashboardTemplate{
//     pub(crate) error: Option<String>
// }

#[derive(Template)]
#[template(path = "loans.html")]
pub struct DashboardTemplate{
    pub(crate) email: Option<String>

}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

