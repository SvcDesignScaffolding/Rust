use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

fn query_handler() -> impl Responder {
    let response = Response { message: "Query successful".to_string() };
    HttpResponse::Ok().json(response)
}

fn insert_handler() -> impl Responder {
    let response = Response { message: "Insert successful".to_string() };
    HttpResponse::Ok().json(response)
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/query", web::get().to(query_handler))
            .route("/api/insert", web::post().to(insert_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
