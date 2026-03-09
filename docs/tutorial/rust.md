# Rust

[docs.rs/lnu-elytra](https://docs.rs/lnu-elytra)

[crates.io/lnu-elytra](http://crates.io/crates/lnu-elytra)

## 异步API（推荐使用）

### 安装

```sh
cargo add tokio -F macros,rt-multi-thread
cargo add lnu-elytra
```

### 使用

```rs
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
```

## 最佳实践

在抢课开始前提前10分钟就登入系统，并等到抢课时间开始时在手动控制脚本继续运行（也可以通过断点调试、Cookie登录实现

### 示例：通过stdin控制脚本继续

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lnu_elytra::Client::new();
    client.login("账号", "密码").await?;

    println!("登录成功，按回车键继续...");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    println!("正在初始化...");

    client.init().await?;
    let course = client.fetch_courses("教学班").await?;
    course.try_select_0(&client).await?;
    Ok(())
}
```

## 阻塞API

::: danger 不要在`tokio`的异步环境中使用阻塞客户端，这会引起panic
Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.
:::

### 安装

需要启用 `blocking` Feature

```sh
cargo add lnu-elytra -F blocking
```

### 使用

```rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lnu_elytra::blocking::Client::new();
    client.login("账号", "密码")?;
    client.init()?;
    let course = client.fetch_course("教学班")?;
    course.try_select_0_blocking(&client)?;
    Ok(())
}
```
