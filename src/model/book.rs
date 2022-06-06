use serde::{Deserialize, Serialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct Book{
    pub id :String,
    pub demand_id:String,
    pub file_path:String,
    pub title:String,
    pub description:String, 
} 
#[derive(Debug,Serialize)]
pub struct Books{
    pub books:Vec<Book>,
}
#[derive(Debug)]
pub struct BookDTO{
    pub demand_id:String,
    pub file_path:String,
    pub title:String,
    pub description:String,
}