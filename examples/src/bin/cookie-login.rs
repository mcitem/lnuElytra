#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lnu_elytra::Client::new();

    // client.login("账号", "密码").await?;
    // 通过cookie登录
    client.set_cookie_override("JSESSIONID=XXX; X-LB=YYY".into());

    client.init().await?;
    let course = client.fetch_courses("教学班").await?;
    course.try_select_0(&client).await?;
    Ok(())
}
