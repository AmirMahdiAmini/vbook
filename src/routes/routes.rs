use actix_web::web::{ServiceConfig, self};
use actix_web::error;
use crate::controller::book::BookController;
use crate::controller::demand::DemandController;
use crate::response;
pub struct Routes{
    pub db :mongodb::sync::Database
}
impl Routes{
    pub fn book_routes(cfg:&mut ServiceConfig){
        cfg.service(web::scope("/book/")
            .app_data(web::JsonConfig::default().error_handler(|_,_|{
                error::InternalError::from_response(
                    "Book Bad Request ",
                    response::bad_request(String::from(r#"{"error":"wrong json data"}"#))
            ).into()
            }))
            .route("/upload", web::post().to(BookController::upload))
            .route("/all", web::get().to(BookController::all))
            .route("/find/{id}", web::get().to(BookController::find))
        );
    }
    pub fn demand_routes(cfg:&mut ServiceConfig) {
        cfg.service(web::scope("/demand/")
        .app_data(web::JsonConfig::default().error_handler(|_,_|{
            error::InternalError::from_response(
                "Demand Bad Request ",
                response::bad_request(String::from(r#"{"error":"wrong json data"}"#))
            ).into()
        }))
        .route("/add", web::post().to(DemandController::add))
        .route("/all", web::get().to(DemandController::all))
        .route("/search", web::get().to(DemandController::search))
        );
    }
}