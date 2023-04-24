use reqwest::Client;

#[tokio::test]
async fn test_health_check() {
    spawn_app();

    let client = Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Could not reach application");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run(0).expect("Could not start server");
    let _ = tokio::spawn(server);
}
