pub async fn health() -> &'static str {
    "Service is up and running"
}

#[cfg(test)]
mod tests {
    use super::health;

    #[tokio::test]
    async fn should_return_ok_for_healthcheck() {
        let response = health().await;

        assert_eq!(response, "Service is up and running");
    }
}
