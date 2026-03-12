use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let mut client = lnu_elytra::Client::new();

    // client.login("账号", "密码").await?;
    // 通过cookie登录
    client.set_cookie_override("JSESSIONID=XXX; X-LB=YYY".into());

    println!("登录成功，按回车键继续...");
    std::io::stdin().read_line(&mut String::new())?;
    println!("正在初始化...");

    client.init().await?;
    let course = client.fetch_courses("教学班").await?;
    course.try_select_0(&client).await?;
    Ok(())
}
