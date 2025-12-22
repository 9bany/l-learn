use actix_web::{App, HttpServer};
use api::{hello, index};
use config::{Config, from_env};
use env_logger;
use env_logger::Builder;
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

    let mut builder = Builder::new();
    builder.filter_level(config.app.get_log_level());
    builder.init();

    println!("Environment: {}", config.env);
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8080))?
        .workers(2)
        .run()
        .await
}
