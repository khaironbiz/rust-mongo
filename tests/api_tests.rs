use axum::{body::Body, http::{Request, StatusCode}};
use tower::util::ServiceExt;

#[tokio::test]
async fn get_doctors_returns_ok() {
    dotenvy::dotenv().ok();
    let state = rme_api_rust::db::init_db().await.expect("db init");
    let app = rme_api_rust::routes::create_router(state);

    // This route is protected by auth middleware, so it should return 401 without a token.

    let req = Request::builder()
        .method("GET")
        .uri("/doctors")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.expect("request failed");
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_medical_records_returns_ok() {
    dotenvy::dotenv().ok();
    let state = rme_api_rust::db::init_db().await.expect("db init");
    let app = rme_api_rust::routes::create_router(state);

    // This route is protected by auth middleware, so it should return 401 without a token.

    let req = Request::builder()
        .method("GET")
        .uri("/medical-records")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.expect("request failed");
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
