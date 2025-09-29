use actix_web::{web, HttpResponse, Result};

pub async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello, World!"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello", web::get().to(hello));
}