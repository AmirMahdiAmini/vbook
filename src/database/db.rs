use mongodb;

pub struct MongoDatabase;

impl MongoDatabase{
    pub fn connect()-> Result<mongodb::sync::Database,Box<dyn std::error::Error>>{
        let db_uri = "DB_URI";
        let clinet = mongodb::sync::Client::with_options(mongodb::options::ClientOptions::parse(&db_uri).unwrap()).expect("couldn't connect to DB");
        Ok(clinet.database("vbook"))
    }
}