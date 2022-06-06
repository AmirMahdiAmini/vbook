use actix_web::{HttpResponse, http::StatusCode};

pub fn ok(json:String)->HttpResponse{
    response(StatusCode::OK,json)
}
pub fn created(json:String)->HttpResponse{
    response(StatusCode::CREATED,json)
}
pub fn not_found(json:String)->HttpResponse{
    response(StatusCode::NOT_FOUND,json)
}
pub fn bad_request(json:String)->HttpResponse{
    response(StatusCode::BAD_REQUEST,json)
}
pub fn default_url_notfound()->HttpResponse{
    response(StatusCode::NOT_FOUND,String::from(r#"{"error":"url not found"}"#))
}
pub fn default_internal_server()->HttpResponse{
    response(StatusCode::INTERNAL_SERVER_ERROR,String::from(r#"{"error":"internal server error, try later a minute"}"#))
}
pub fn default_ok()->HttpResponse{
    response(StatusCode::OK,String::from(r#"{"message":"OK"}"#))
}
fn response(status:StatusCode,json:String)->HttpResponse{
    HttpResponse::build(status)
    .header("Content-Type", "application/json")
    .body(json)
}