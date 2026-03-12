use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::util::ServiceExt;
use why_is_it_green_api::app;

#[tokio::test]
async fn should_return_ok_on_version_endpoint() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/version")
                .body(Body::empty())
                .expect("request should be built"),
        )
        .await
        .expect("router should handle request");

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get(header::CONTENT_TYPE),
        Some(&header::HeaderValue::from_static("application/json"))
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should be readable")
        .to_bytes();

    let body: Value = serde_json::from_slice(&body).expect("response body should be valid json");

    assert_eq!(body["version"], "0.0.3");
    assert!(body["environment"].is_string());
    assert!(body["build_number"].is_string());
    assert!(body["git_sha"].is_string());
}
