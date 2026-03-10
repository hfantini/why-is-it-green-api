use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::util::ServiceExt;
use why_is_it_green_api::app;

#[tokio::test]
async fn should_return_ok_on_healthcheck_endpoint() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/healthcheck")
                .body(Body::empty())
                .expect("request should be built"),
        )
        .await
        .expect("router should handle request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should be readable")
        .to_bytes();

    assert_eq!(body.as_ref(), b"Service is up and running");
}
