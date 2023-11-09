// src/main.rs

use actix_web::{web, App, HttpServer};
use clickhouse::{Client, Row};
use log::{error, info};
use tracing::{info_span, Instrument};
use tracing_subscriber::{layer::Context, prelude::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志和跟踪
    let subscriber = tracing_subscriber::fmt().with_env_filter("info");
    let (layer, _guard) = Context::current().layer(&subscriber);

    // 创建 ClickHouse 客户端
    let client = Client::new("localhost:9000");

    // 创建 HTTP 服务器
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(layer)
            .service(web::resource("/logql").route(web::get().to(logql_handler)))
            .service(web::resource("/promql").route(web::get().to(promql_handler)))
            .service(web::resource("/tracing").route(web::get().to(tracing_handler)))
    });

    // 启动 HTTP 服务器
    server.bind("0.0.0.0:8080").unwrap().run().await
}

async fn logql_handler() -> impl actix_web::Responder {
    // 获取查询字符串
    let query = web::Query::<String>::from_query().await.unwrap();

    // 执行查询
    let rows = client.query(query.as_str()).await.unwrap();

    // 构建响应
    let response = serde_json::to_string_pretty(&rows).unwrap();
    HttpResponse::Ok().body(response)
}

async fn promql_handler() -> impl actix_web::Responder {
    // 获取查询字符串
    let query = web::Query::<String>::from_query().await.unwrap();

    // 执行查询
    let rows = client.query(query.as_str(), &"PromQL").await.unwrap();

    // 构建响应
    let response = serde_json::to_string_pretty(&rows).unwrap();
    HttpResponse::Ok().body(response)
}

async fn tracing_handler() -> impl actix_web::Responder {
    // 获取 trace ID
    let trace_id = web::Query::<String>::from_query().await.unwrap();

    // 查询 trace
    let rows = client.query(
        "SELECT * FROM tracing WHERE trace_id = ?",
        &[trace_id],
    ).await.unwrap();

    // 构建响应
    let response = serde_json::to_string_pretty(&rows).unwrap();
    HttpResponse::Ok().body(response)
}
