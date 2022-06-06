use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Demand{
    pub id :String,
    pub name:String,
    pub book:String,
    pub information:String,
    pub importance:Importance,
    pub option:DemandOption,
    pub book_id:String,
    pub created_at: String,
}
impl Demand{
    pub fn new(name:String,book:String,information:String,importance:Importance,option:DemandOption)->Self{
        Self{
            id:String::from(""),
            name,
            book,
            information,
            book_id:String::from(""),
            importance,
            option,
            created_at:chrono::Utc::now().to_string()
        }
    }
}
#[derive(Debug,Serialize)]
pub struct Demands{
    pub demands:Vec<Demand>,
}
#[derive(Deserialize,Debug)]
pub struct DemandDST{
    pub book:String,
}
#[derive(Deserialize,Debug)]
pub struct DemandDTO{
    pub name:String,
    pub book:String,
    pub information:String,
    pub importance:Importance,
    pub option:DemandOption
}
#[derive(Serialize,Deserialize,Debug)]
pub enum Importance{
    HIGH,
    NORMAL,
    LOW,
}
#[derive(Serialize,Deserialize,Debug)]
pub enum DemandOption{
    ADD,
    DELETE,
    EDIT,
}
impl Importance{
    pub fn as_str(&self) -> &'static str {
        match self {
            Importance::HIGH => "HIGH",
            Importance::NORMAL => "NORMAL",
            Importance::LOW => "LOW",
        }
    }
}

impl DemandOption{
    pub fn as_str(&self) -> &'static str {
        match self {
            DemandOption::ADD => "ADD",
            DemandOption::DELETE => "DELETE",
            DemandOption::EDIT => "EDIT",
        }
    }
}