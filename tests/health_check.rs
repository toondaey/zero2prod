use reqwest::Client;
use test_case::test_case;
use urlencoding::encode;

mod common;

use common::ConfigureTestContext;

#[tokio::test]
async fn test_health_check() {
    let ctx = ConfigureTestContext::setup().await;

    let client = Client::new();
    let response = client
        .get(format!("http://localhost:{}/health_check", ctx.port))
        .send()
        .await
        .expect("Could not reach application");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    ctx.teardown().await;
}

#[tokio::test]
async fn test_subscriptions_passes() {
    let name = "Babatunde Aromire";
    let email = "aromire.tunde@gmail.com";
    let status_code = 200u16;
    let message = "";
    let ctx = ConfigureTestContext::setup().await;
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
        .post(format!("http://localhost:{}/subscriptions", ctx.port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_body)
        .send()
        .await
        .expect("Could not reach application");

    assert_eq!(response.status().as_u16(), status_code, "{message}");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions;")
        .fetch_one(&ctx.connection_pool)
        .await
        .unwrap();
    assert_eq!("Babatunde Aromire", saved.name);
    assert_eq!("aromire.tunde@gmail.com", saved.email);

    ctx.teardown().await;
}

#[test_case("", "aromire.tunde@gmail.com", 400, "missing name")]
#[test_case("Babatunde Aromire", "", 400, "missing email")]
#[tokio::test]
async fn test_subscriptions_fails(name: &str, email: &str, status_code: u16, message: &str) {
    let ctx = ConfigureTestContext::setup().await;
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
        .post(format!("http://localhost:{}/subscriptions", ctx.port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_body)
        .send()
        .await
        .expect("Could not reach application");

    assert_eq!(response.status().as_u16(), status_code, "{message}");

    ctx.teardown().await;
}
