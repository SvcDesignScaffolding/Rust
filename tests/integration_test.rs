use actix_rt::System;
use actix_web::{test, App, http};

#[actix_rt::test]
async fn test_query_api() {
    let mut app = test::init_service(App::new().route("/api/query", web::get().to(crate::query_handler))).await;
    let req = test::TestRequest::get().uri("/api/query").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_insert_api() {
    let mut app = test::init_service(App::new().route("/api/insert", web::post().to(crate::insert_handler))).await;
    let req = test::TestRequest::post().uri("/api/insert").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
