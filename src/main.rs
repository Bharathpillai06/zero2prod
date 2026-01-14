use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn health_check() -> impl Responder {
HttpResponse::Ok()
}



async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
#[cfg(test)]
mod tests {
use crate::health_check;
#[tokio::test]
async fn health_check_succeeds() {
let response = health_check().await;
// This requires changing the return type of `health_check`
// from `impl Responder` to `HttpResponse` to compile
// You also need to import it with `use actix_web::HttpResponse`!
assert!(response.status().is_success())
}
}