# Rust

[docs.rs/lnu-elytra](https://docs.rs/lnu-elytra)

[crates.io/lnu-elytra](http://crates.io/crates/lnu-elytra)

安装Rust工具链 [安装 Rust](https://rust-lang.org/zh-CN/tools/install/)

```sh
git clone https://github.com/mcitem/lnuElytra
cd examples
# 修改 examples/src/bin/best中的教学班、账号密码
cargo run --bin best
```

## 异步API（推荐使用）

### 安装

```sh
cargo add tokio -F macros,rt-multi-thread
cargo add lnu-elytra
```

### 使用

<<< @/../examples/src/bin/quick-start.rs

## 最佳实践

在抢课开始前提前10分钟就登入系统，并等到抢课时间开始时在手动控制脚本继续运行（也可以通过断点调试、Cookie登录实现

### 示例：通过stdin控制脚本继续

<<< @/../examples/src/bin/stdin-control.rs

### 完整示例(可直接使用)

<<< @/../examples/src/bin/best.rs

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

<<< @/../examples/src/bin/blocking.rs
