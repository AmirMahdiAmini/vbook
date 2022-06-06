
#[path = "./controller/mod.rs"]
pub mod controller;
#[path = "./service/mod.rs"]
pub mod service;
#[path = "./database/db.rs"]
pub mod database;
#[path = "./model/mod.rs"]
pub mod model;
#[path = "./routes/routes.rs"]
pub mod routes;
pub mod response;


pub static mut DATABASE:Option<mongodb::sync::Database> =None;
