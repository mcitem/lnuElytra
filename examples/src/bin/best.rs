use std::sync::Arc;

use lnu_elytra::{Client, Error, SelectCourseResponse};
use tracing_subscriber::{filter::LevelFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt().with_max_level(LevelFilter::INFO).init();

    // 此示例非常适合直接用于单人抢课使用，含有基本的重试，多任务并行功能

    let mut client = Client::new();

    // 在 广州商学院 抢课（更换教务系统地址
    // let mut client = Client::new_with_base("http://jwxt.gcc.edu.cn".try_into()?);

    client.login("账号", "密码").await?;

    println!("登录成功，按回车键继续...");

    // 如果需要手动控制，取消注释此行
    // std::io::stdin().read_line(&mut String::new())?;

    println!("正在初始化...");

    // 登录成功后，永远不要引发任何的panic或直接上抛Result Err，否则会导致所有直接退出，无法继续抢课

    loop {
        if let Ok(()) = client.init().await {
            println!("初始化成功，正在准备选课...");
            break;
        };
        println!("初始化失败，正在重试...");
    }

    // 教学班列表
    // 并行运行多个选课任务，但可能会随机选到其中一个
    let tgs = vec!["教学班1", "教学班2", "教学班3"];

    let mut hds = Vec::new();

    // 使用std::sync::Arc来共享Client实例，以便在多个任务中使用

    let c = Arc::new(client);

    for i in tgs {
        hds.push(tokio::spawn(fetch(i, c.clone())));
    }

    for i in hds {
        if let Ok(status) = i.await {
            println!("选课结果: {:?}", status);
        }
    }

    println!("进程已结束，按回车键退出...");
    std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}

async fn fetch(i: &str, client: Arc<Client>) -> Result<SelectCourseResponse, Error> {
    println!("正在选课 {}...", i);
    loop {
        if let Ok(course) = client.fetch_courses(i).await
            && let Ok(status) = course.try_select_0(&client).await
        {
            println!("选课结果 {}: {:?}", i, status);

            if let Some(msg) = status.msg()
                && (msg.contains("未开放") || msg.contains("频率过高"))
            {
                continue;
            }

            if status.is_success() {
                println!("选课成功 {}!", i);
            }

            return Ok(status);
        };
    }
}
