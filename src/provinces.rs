use std::ops::Not;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
struct Province {
    id: i32,
    name: String,
    code: String,
}

impl Province {
    fn new(id: i32, name: &str, code: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            code: code.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct QueryInfo {
    country_code: String,
}



#[get("/provinces")]
async fn list_provinces(web::Query(code): web::Query<QueryInfo>) -> impl Responder {
    if code.country_code.is_empty().not() {
        HttpResponse::Ok().json(get_provinces(code.country_code))
    } else {
        HttpResponse::NotFound().finish()
    }
}


fn get_provinces(country_code: String) -> Vec<Province>  {
    // In a real implementation, you would query a database or external API to get the list of provinces
    // For the sake of this example, we'll just return a hardcoded list of provinces for the United States
    if country_code == "US" {
        vec![
            Province::new(1, "Alabama", "AL"),
            Province::new(2, "Alaska", "AK"),
            Province::new(3, "Arizona", "AZ"),
            // ... add more provinces here
        ]
    } else if country_code == "IN" {
        vec![
            Province::new(1, "Madhya Pradesh", "MP"),
            Province::new(2, "Uttar Pradesh", "UP"),
            Province::new(3, "Kerala", "KL"),
            // ... add more provinces here
        ]
    } else if country_code == "AU" {
        vec![
            Province::new(1, "New South Wales", "NSW"),
            Province::new(2, "Victoria", "VIC"),
            Province::new(3, "Queensland", "QLD"),
            // ... add more provinces here
        ]
    } else if country_code == "UK" {
        vec![
            Province::new(1, "Leeds", "LDS"),
            Province::new(2, "Liverpool", "LIV"),
            Province::new(3, "Manchester", "MAN"),
            // ... add more provinces here
        ]
    } else {
        vec![
            Province::new(1, "Madhya Pradesh", "MP"),
            Province::new(2, "Uttar Pradesh", "UP"),
            Province::new(3, "Kerala", "KL"),
            // ... add more provinces here
        ]
    }
}

