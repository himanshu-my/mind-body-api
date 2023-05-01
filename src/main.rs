use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
mod countries;
mod provinces;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(countries::list_countries)
            .service(provinces::list_provinces);
    };

    Ok(config.into())
}
