use actix_web::{web, App, HttpResponse, HttpServer};

// this function could be located in different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}