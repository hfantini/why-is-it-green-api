use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
struct VersionResponse {
    version: &'static str,
    environment: String,
    build_number: String,
    git_sha: String,
}

fn build_version_response(
    environment: Option<&str>,
    build_number: Option<&str>,
    git_sha: Option<&str>,
) -> VersionResponse {
    VersionResponse {
        version: env!("CARGO_PKG_VERSION"),
        environment: environment.unwrap_or("?").to_owned(),
        build_number: build_number.unwrap_or("?").to_owned(),
        git_sha: git_sha.unwrap_or("?").to_owned(),
    }
}

pub async fn version() -> impl IntoResponse {
    Json(build_version_response(
        std::env::var("ENVIRONMENT").ok().as_deref(),
        std::env::var("BUILD_NUMBER").ok().as_deref(),
        std::env::var("GIT_SHA").ok().as_deref(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{VersionResponse, build_version_response};

    #[test]
    fn should_build_version_response_with_provided_values() {
        let response = build_version_response(Some("staging"), Some("123"), Some("abc123"));

        assert_eq!(
            response,
            VersionResponse {
                version: "0.0.3",
                environment: "staging".to_owned(),
                build_number: "123".to_owned(),
                git_sha: "abc123".to_owned(),
            }
        );
    }

    #[test]
    fn should_build_version_response_with_unknown_defaults() {
        let response = build_version_response(None, None, None);

        assert_eq!(
            response,
            VersionResponse {
                version: "0.0.3",
                environment: "unknown".to_owned(),
                build_number: "unknown".to_owned(),
                git_sha: "unknown".to_owned(),
            }
        );
    }

    #[test]
    fn should_build_non_empty_version_response() {
        let response = build_version_response(None, None, None);

        assert_eq!(response.version, "0.0.3");
        assert!(!response.environment.is_empty());
        assert!(!response.build_number.is_empty());
        assert!(!response.git_sha.is_empty());
    }
}
