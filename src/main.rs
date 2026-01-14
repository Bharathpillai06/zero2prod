use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn health_check(req: HttpRequest) -> impl Responder {
HttpResponse::Ok().finish()
}



async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // health check endpoint
            .route("/health_check", web::get().to(health_check))
            // greeting endpoints
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}