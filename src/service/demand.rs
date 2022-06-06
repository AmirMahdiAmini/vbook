
use actix_web::{HttpResponse};
use bson;
use crate::{model::demand::{DemandDTO,Demand, Demands},response};
use crate::DATABASE;

#[derive(Debug,Clone)]

pub struct DemandService;
impl DemandService{
    pub fn add(dto:DemandDTO)->HttpResponse{
        let demand = Demand::new(dto.name, dto.book, dto.information, dto.importance,dto.option);
        let collection:mongodb::sync::Collection<bson::Document>;
        unsafe{
            collection = DATABASE.clone().expect("There's no database on this variable").collection("demand");
        }
        let result = &collection.insert_one(bson::doc! {
            "id":uuid::Uuid::new_v4().to_simple().to_string(),
            "name":&demand.name,
            "book":&demand.book,
            "information":&demand.information,
            "book_id":"NULL",
            "importance":&demand.importance.as_str(), 
            "option":&demand.option.as_str(),
            "created_at":&demand.created_at.to_string()
           
        }, None);
        match &result{
            Ok(_) => response::default_ok(),
            Err(_) => response::bad_request(String::from(r#"{"error":"couldn't add your data. try later"}"#))
        }
    }
    pub fn all()->HttpResponse{
        let mut data:Vec<Demand> = vec![];
        let collection:mongodb::sync::Collection<bson::Document>;
        unsafe{
            collection = DATABASE.clone().expect("There's no database on this variable").collection("demand");
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
                    let demand:Demand = bson::from_bson(bson::Bson::Document(docu)).unwrap();
                    data.push(demand);
                },
                Err(_) =>  return response::bad_request(String::from(r#"{"error":"try later"}"#))
            }
        }
       
        let demands = Demands{
            demands:data,
        };
        response::ok(serde_json::to_string(&demands).unwrap())
    }

    pub fn search(demand_name:String)->HttpResponse{
        let mut data:Vec<Demand> = vec![];
        let collection:mongodb::sync::Collection<bson::Document>;
        unsafe{
            collection = DATABASE.clone().expect("There's no database on this variable").collection("demand");
        }
        let query = bson::doc!{"book":{"$regex":demand_name}};
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
                    let demand:Demand = bson::from_bson(bson::Bson::Document(docu)).unwrap();
                    data.push(demand);
                },
                Err(_) =>  return response::bad_request(String::from(r#"{"error":"try later"}"#))
            }
        }
        let demands = Demands{
            demands:data,
        };
        response::ok(serde_json::to_string(&demands).unwrap())
    }
}
