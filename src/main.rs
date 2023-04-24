use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> ::std::io::Result<()> {
    let listener = TcpListener::bind("http://127.0.0.1:0").expect("Failed to find random port");
    run(listener)?.await
}
