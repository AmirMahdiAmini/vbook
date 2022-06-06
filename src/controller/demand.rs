use actix_web::{HttpResponse, web::Json, HttpRequest};
use crate::{model::demand::{DemandDTO, DemandDST},service::demand::DemandService};
pub struct DemandController;
impl DemandController{
    pub async fn add(req : HttpRequest,data: Json<DemandDTO>)->HttpResponse{
        println!("\x1b[44;97m {} \x1b[0m \x1b[41;97m {} \x1b[0m \x1b[43;97m request to [Demand] => ADD \x1b[0m",req.peer_addr().unwrap(),req.method());
        DemandService::add(Json::into_inner(data))
    }
    pub async fn all(req : HttpRequest)->HttpResponse{
        println!("\x1b[44;97m {} \x1b[0m \x1b[41;97m {} \x1b[0m \x1b[43;97m request to [Demand] => ALL \x1b[0m",req.peer_addr().unwrap(),req.method());
        DemandService::all()
    }

    pub async fn search(req : HttpRequest,data:Json<DemandDST>)->HttpResponse{
        println!("\x1b[44;97m {} \x1b[0m \x1b[41;97m {} \x1b[0m \x1b[43;97m request to [Demand] => SEARCH \x1b[0m",req.peer_addr().unwrap(),req.method());
        DemandService::search(Json::into_inner(data).book)
    }
}