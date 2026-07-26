#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lnu_elytra::Client::new();

    // client.login("账号", "密码").await?;
    // 通过cookie登录

    // insert_cookie: 整条字符串作为 Set-Cookie 解析，仅写入第一条 Cookie
    // client.insert_cookie("JSESSIONID=XXX")?;

    // insert_cookies: 按 ; 拆分，逐条写入，适合浏览器复制的多条 Cookie
    client.insert_cookies("JSESSIONID=XXX; zstack_cookie=YYY")?;

    client.check_login().await?;

    client.init().await?;
    let course = client.fetch_courses("教学班").await?;
    course.try_select_0(&client).await?;
    Ok(())
}
