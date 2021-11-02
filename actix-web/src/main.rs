use actix_web::{web, App, get, HttpServer, HttpRequest, HttpResponse, Responder};

async fn index() -> impl Responder {
    "Hello"
}

async fn about() -> impl Responder {
    "About"
}

#[get("/my")]
async fn my() -> impl Responder {
    "my"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
            .service(my)
    })
    .bind("0.0.0.0:10001")?
    .run()
    .await
}