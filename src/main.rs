mod api;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .configure(api::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
