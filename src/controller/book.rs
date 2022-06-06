
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Result, Error, HttpRequest, web};
use crate::{service::book::BookService};

pub struct BookController;
impl BookController{
    pub async fn upload(req : HttpRequest,payload: Multipart) -> Result<HttpResponse, Error> {
        println!("\x1b[44;97m {} \x1b[0m \x1b[41;97m {} \x1b[0m \x1b[43;97m request to [Book] => UPLOAD \x1b[0m",req.peer_addr().unwrap(),req.method());

        BookService::save_file(payload).await
    }
    pub fn all(req : HttpRequest) -> HttpResponse {
        println!("\x1b[44;97m {} \x1b[0m \x1b[41;97m {} \x1b[0m \x1b[43;97m request to [Book] => ALL \x1b[0m",req.peer_addr().unwrap(),req.method());
        BookService::all()
    }
    pub fn find(req : HttpRequest,path : web::Path<String>) -> HttpResponse {
        println!("\x1b[44;97m {} \x1b[0m \x1b[41;97m {} \x1b[0m \x1b[43;97m request to [Book] => FIND \x1b[0m",req.peer_addr().unwrap(),req.method());
        BookService::find(path.0)
    }
}