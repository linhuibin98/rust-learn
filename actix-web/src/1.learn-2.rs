use actix_web::{web, App, HttpServer, HttpRequest, Responder};

async fn home_control(req: HttpRequest) -> impl Responder {
    "Home"
}

async fn about_control(req: HttpRequest) -> impl Responder {
    "About"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home_control))
            .route("/about", web::get().to(about_control))
    })
    .bind("127.0.0.1:8346")?
    .run()
    .await
}