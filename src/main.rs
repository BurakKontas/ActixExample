use actix_web::{ App, HttpServer };
use controllers::hello;

pub mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Sunucu başlatılmadan önce bir mesaj yazdırın
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello::greet_json)
            .service(hello::greet_path)
            .service(hello::greet_query)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}