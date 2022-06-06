use actix_multipart::Multipart;
use actix_web::web;
use actix_web::{Error,HttpResponse};
use futures_util::{stream::StreamExt as _, TryStreamExt};
use std::fs;
use std::io::Write;
use crate::DATABASE;
use crate::response;
use crate::model::book::{BookDTO, Book, Books};
pub struct BookService;

impl BookService{
    pub async fn save_file(mut payload:Multipart)->Result<HttpResponse,Error>{
        let mut book =  BookDTO{
            demand_id:String::from(""),          
            title:String::from(""),
            description:String::from(""),
            file_path:String::from(""),
        };
        while let Some(item) = payload.next().await {
            let mut field = item?;
        if field.content_disposition().unwrap().get_name() == Some("file"){
            
            if field.content_disposition().unwrap().get_filename().unwrap().split(".").last().unwrap() != "pdf" 
            {
                return Err(response::bad_request(String::from(r#"{"error":"please send the correct form"}"#)).into())
            }
            let path = format!("./upload/{}.{}",uuid::Uuid::new_v4().to_simple().to_string()
            ,&field.content_disposition().unwrap().get_filename().unwrap().split(".").last().unwrap());
            book.file_path = path.clone();
            let mut file = web::block(move || std::fs::File::create(&path))
                .await
                .unwrap();
            while let Some(chunk) = field.try_next().await? {
                file = web::block(move || file.write_all(&chunk).map(|_| file))
                            .await
                            .unwrap();
            }
        }else{
            while let Some(chunk) = field.try_next().await? {
                if field.content_disposition().unwrap().get_name() == Some("title"){
                    book.title = String::from_utf8(chunk.to_vec()).unwrap();
                } 
                if field.content_disposition().unwrap().get_name() == Some("id"){
                    book.demand_id = String::from_utf8(chunk.to_vec()).unwrap();
                } 
                if field.content_disposition().unwrap().get_name() == Some("description"){
                    book.description = String::from_utf8(chunk.to_vec()).unwrap();
                };
            }
        };
            
        }
        if book.demand_id.is_empty(){
            fs::remove_file(book.file_path.clone()).unwrap();
            return Err(response::bad_request(String::from(r#"{"error":"please send the correct form"}"#)).into())
        }
        
        let book_collection:mongodb::sync::Collection<bson::Document>;
        let demand_collection:mongodb::sync::Collection<bson::Document>;
        unsafe{
            book_collection = DATABASE.clone().expect("There's no database on this variable").collection("book");
            demand_collection = DATABASE.clone().expect("There's no database on this variable").collection("demand");
        }
        let find = demand_collection.find_one(bson::doc!{"id":book.demand_id.clone()}, None).unwrap();
        match &find{
            Some(_) => {},
            None => {
                fs::remove_file(book.file_path.clone()).unwrap();
                return Err(response::bad_request(String::from(r#"{"error":"please send the correct form"}"#)).into())
            },
        };
        let book_id = uuid::Uuid::new_v4().to_simple().to_string();
        let result = book_collection.insert_one(bson::doc! {
            "id":&book_id.clone(),
            "demand_id":&book.demand_id,
            "title":&book.title,
            "description":&book.description,
            "file_path":&book.file_path,
        }, None);
        
        match &result{
            Ok(_) => {
                demand_collection.update_one(bson::doc!{"id":book.demand_id.clone()}, bson::doc!{"$set":{"book_id":book_id}}, None).unwrap();
                Ok(response::created(String::from(r#"{"message":"your form is submitted"}"#)))
            },
            Err(_) => Err(response::bad_request(String::from(r#"{"error":"couldn't add your data. try later"}"#)).into())
        }
    }
    pub fn all()->HttpResponse{
        let mut data:Vec<Book> = vec![];
        let collection:mongodb::sync::Collection<bson::Document>;
        unsafe{
            collection = DATABASE.clone().expect("There's no database on this variable").collection("book");
        }
        let result = collection.find(None, None);
        let result = match result{
            Ok(e) =>e,
            Err(e) => {
                println!("{:?}",e);
                return response::bad_request(String::from(r#"{"error":"try later"}"#))}
        };
        for cursor in result{
            match cursor{
                Ok(docu) => {
                    let book:Book = bson::from_bson(bson::Bson::Document(docu)).unwrap();
                    data.push(book);
                },
                Err(_) =>  return response::bad_request(String::from(r#"{"error":"try later"}"#))
            }
        }
       
        let books = Books{
            books:data,
        };
        response::ok(serde_json::to_string(&books).unwrap())
    }
    pub fn find(book_id:String)->HttpResponse{
        let mut data:Vec<Book> = vec![];
        let collection:mongodb::sync::Collection<bson::Document>;
        unsafe{
            collection = DATABASE.clone().expect("There's no database on this variable").collection("book");
        }
        let query = bson::doc!{"demand_id":book_id};
        let result = collection.find(query, None);
        let result = match result{
            Ok(e) =>e,
            Err(e) => {
                println!("{:?}",e);
                return response::bad_request(String::from(r#"{"error":"try later"}"#))}
        };
        for cursor in result{
            match cursor{
                Ok(docu) => {
                    let book:Book = bson::from_bson(bson::Bson::Document(docu)).unwrap();
                    data.push(book);
                },
                Err(_) =>  return response::bad_request(String::from(r#"{"error":"try later"}"#))
            }
        }
        let books = Books{
            books:data,
        };
        response::ok(serde_json::to_string(&books).unwrap())
    }
}

