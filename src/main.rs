use std::io::Result;

use actix_web::{HttpServer, App, web};

pub mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Server listening at http://localhost:5532");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(handlers::ping_handler::ping)
        )
    })
    .bind(("127.0.0.1", 5532))?
    .run()
    .await
}
