use actix_web::{get, App, HttpServer, Responder};
use rand::Rng;

#[get("/")]
async fn random_number() -> impl Responder {
    let number = rand::thread_rng().gen_range(1..=100000);
    number.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(random_number))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}