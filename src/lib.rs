use actix_web::{get, web::ServiceConfig};
use shuttle_service::ShuttleActixWeb;
mod countries;
mod provinces;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

// #[shuttle_service::main]
// async fn actix_web(
// ) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
//     Ok(move |cfg: &mut ServiceConfig| {
//         cfg.service(countries::list_countries)
//         .service(provinces::list_provinces);
//     })
// }
