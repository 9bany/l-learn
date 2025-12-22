use actix_web::{App, HttpServer, Responder, get, web};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}