fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lnu_elytra::blocking::Client::new();
    client.login("账号", "密码")?;
    client.init()?;
    let course = client.fetch_course("教学班")?;
    course.try_select_0_blocking(&client)?;
    Ok(())
}
