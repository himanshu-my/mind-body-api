use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::get;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Country {
    id: i32,
    name: String,
    code: String,
    phone_code: i32,
}

impl Country {
    fn new(id: i32, name: &str, code: &str, phone_code: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            code: code.to_string(),
            phone_code,
        }
    }
}   


#[get("/countries")]
async fn list_countries() -> impl Responder {
    let countries = get_countries();
    HttpResponse::Ok().json(countries)
}


fn get_countries() -> Vec<Country> {
    vec![
        Country::new(1, "United States", "US", 1),
        Country::new(2, "Australia", "AU", 61),
        Country::new(3, "England", "UK", 44),
        Country::new(4, "India", "IN", 91),
        // ... add more countries here
    ]
}
