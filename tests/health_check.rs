use std::net::TcpListener;

use reqwest::Client;
use test_case::test_case;
use urlencoding::encode;

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

#[test_case("Babatunde Aromire", "aromire.tunde@gmail.com", 200, "")]
#[test_case("", "aromire.tunde@gmail.com", 400, "missing name")]
#[test_case("Babatunde Aromire", "", 400, "missing email")]
#[tokio::test]
async fn test_subscriptions(name: &str, email: &str, status_code: u16, message: &str) {
    let port = spawn_app();

    let encoded_name = encode(name);
    let encoded_email = encode(email);
    let mut encoded_body = "".to_owned();

    if encoded_name.len() > 0 {
        encoded_body.push_str(&format!("name={}", &encoded_name));
    }
    if encoded_email.len() > 0 {
        encoded_body.push_str(&format!("&email={}", &encoded_email));
    }

    let client = Client::new();
    let response = client
        .post(format!("http://localhost:{port}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_body)
        .send()
        .await
        .expect("Could not reach application");

    assert_eq!(response.status().as_u16(), status_code, "{message}");
}

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Could not start server");
    let _ = tokio::spawn(server);
    port
}
