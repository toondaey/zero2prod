use zero2prod::run;

#[tokio::main]
async fn main() -> ::std::io::Result<()> {
    run(8000)?.await
}

