use actix_web::{App, HttpServer};
use api::{hello, index};
use config::{Config, from_env};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config: Config;
    match from_env() {
        Ok(c) => {
            config = c;
        }
        Err(error) => panic!("{:#?}", error),
    }

    println!("{:#?}", config.app.log_level);
    // log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8080))?
        .workers(2)
        .run()
        .await
}
