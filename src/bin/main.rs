use actix_web::{HttpServer,App,HttpResponse,web, Result};
use voolibowlib::DATABASE;
use voolibowlib::{routes::Routes, response,database::MongoDatabase,};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("\x1b[44;97m  MongoDB Started  \x1b[0m");
    unsafe {
        DATABASE =match MongoDatabase::connect(){
            Ok(d) => Some(d),
            Err(_) => panic!("couldn't connect to the database")
        }
    }
    println!("\x1b[44;97m  Server Started  \x1b[0m");
    let address:String = String::from("0.0.0.0:8080");
    HttpServer::new(move||{
        App::new()
        .default_service(
            web::route().to(not_found)
        )
        .configure(Routes::book_routes)
        .configure(Routes::demand_routes)
    })
    .bind(&address)
    .unwrap_or_else(|_| panic!("🔥Could not bind server🔥"))
    .run()
    .await
}
async fn not_found() -> Result<HttpResponse> {
    Ok(response::default_url_notfound())
}

