use actix_web::{ get, post, web, Responder };
use greeter_service::greeter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GreetPathParams {
    name: String,
    surname: String
}

#[get("/{name}/{surname}")]
pub async fn greet_path(path: web::Path<GreetPathParams>) -> impl Responder {
    let name = &path.name;
    let surname = &path.surname;
    greeter(&format!("{} {}", name, surname))
}

#[get("/")]
pub async fn greet_query(params: web::Query<GreetPathParams>) -> impl Responder {
    let name = &params.name;
    let surname = &params.surname;
    greeter(&format!("{} {}", name, surname))
}

#[post("/")]
pub async fn greet_json(params: web::Json<GreetPathParams>) -> impl Responder {
    let name = &params.name;
    let surname = &params.surname;
    greeter(&format!("{} {}", name, surname))
}