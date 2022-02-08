mod config;

use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer};
use config::Config;

#[rustfmt::skip]
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load("./config.toml").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./web/static").prefer_utf8(true))
            .service(Files::new("/wasm", "./web/pkg").prefer_utf8(true))
            .route("*", web::get().to(|| async { NamedFile::open("./web/index.html") }))
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}
