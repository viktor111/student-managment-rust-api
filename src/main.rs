use actix_web::{App, HttpServer};
use actix_web::{middleware::Logger};

use std::{env, io};


mod students;
mod response;
mod constants;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(students::get)
            .service(students::create)
            .service(students::all)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
