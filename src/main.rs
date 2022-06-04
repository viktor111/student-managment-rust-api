use actix_web::{App, HttpServer};

mod students;
mod response;
mod constants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(students::get)
            .service(students::create)
            .service(students::all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
