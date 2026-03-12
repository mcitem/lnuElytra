#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lnu_elytra::Client::new();
    client.login("账号", "密码").await?;
    client.init().await?;

    // 教学班示例：(2025-2026-2)-77101504-02
    // 使用精确的教学班查询能能减少教务系统返回的数据量，有利于加快抢课。
    let course = client.fetch_courses("教学班").await?;
    // 只有当使用精确教学班查询时，才适合直接调用 try_select_o
    course.try_select_0(&client).await?;
    Ok(())
}
