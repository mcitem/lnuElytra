use lnu_elytra::Client;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("lnu_elytra=trace".parse().unwrap()),
        )
        .init();

    let client = Client::new_with_base("http://jwxt.gcc.edu.cn".try_into()?);
    let ver = client.ver().await.unwrap();
    println!("ver: {ver:?}");
    Ok(())
}
