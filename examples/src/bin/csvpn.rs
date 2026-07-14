use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("lnu_elytra=trace".parse().unwrap()),
        )
        .init();

    let mut client = lnu_elytra::Client::new();

    client = client.set_converter(lnu_elytra::csvpn_converter);

    client.insert_cookie("wengine_vpn_ticketcsvpn_lingnan_edu_cn=XXX;")?;

    let result = client.jziotlogin().await?;

    println!("jziotlogin result: {result}");

    let result = client.check_login().await?;

    println!("{result}");

    Ok(())
}
