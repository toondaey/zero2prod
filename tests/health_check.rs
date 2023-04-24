use std::net::TcpListener;

use reqwest::Client;

#[tokio::test]
async fn test_health_check() {
    let port = spawn_app();

    let client = Client::new();
    let response = client
        .get(format!("http://localhost:{port}/health_check"))
        .send()
        .await
        .expect("Could not reach application");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Could not start server");
    let _ = tokio::spawn(server);
    port
}
