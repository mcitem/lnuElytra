# API 参考

## `Client` 方法

### `new()`

::: code-group

```rs [Rust]
let mut client = Client::new();
let mut client = Client::new_with_base("http://jwxt.gcc.edu.cn")?;
```

```py [Python]
client = Client()
client = Client(base="http://jwxt.gcc.edu.cn")
```

:::

### `login(username, password)`

- 用途：执行登录流程
- 参数：
  - `username`: 学号
  - `password`: 密码
- 返回：`String` — 登录用户的 `sessionUserKey`

::: code-group

```rs [Rust]
let session_user_key = client.login("账号", "密码")?;
```

```py [Python]
session_user_key = client.login("账号", "密码")
```

:::

### `check_login()`

- 用途：检查当前会话是否已登录
- 返回：`String` — 登录用户的 `sessionUserKey`

::: code-group

```rs [Rust]
let session_user_key = client.check_login()?;
```

```py [Python]
session_user_key = client.check_login()
```

:::

### `jziotlogin()`

- 用途：通过 SSO 单点登录（jziotlogin）完成登录
- 返回：`String` — 登录用户的 `sessionUserKey`
- 仅在 Rust async `Client` 中可用
- 仅限用于csvpn自动授权登录

::: code-group

```rs [Rust]
let session_user_key = client.jziotlogin().await?;
```

:::

### `init()`

- 用途：初始化选课上下文，供后续查询/选课使用

每次使用仅需调用一次，可供同时选多个课程使用

::: warning
如果选课尚未开始，调用会出现`NotyetStarted`错误，需要妥善处理
:::

::: code-group

```rs [Rust]
client.init()?;
```

```py [Python]
client.init()
```

:::

### `fetch_courses(q)`

- 用途：按条件查询课程并返回可操作课程对象
- 参数：
  - `q`: 课程查询关键字,建议使用精确教学班，例如：`(2025-2026-2)-77101504-02`

::: code-group

```rs [Rust]
// blocking
let course = client.fetch_course("(2025-2026-2)-77101504-02")?;
// async
let course = client.fetch_courses("(2025-2026-2)-77101504-02").await?;
```

```py [Python]
course = client.fetch_course("(2025-2026-2)-77101504-02")
# AsyncClient
course = await client.fetch_courses("(2025-2026-2)-77101504-02")
```

:::

### `select_course(course_id, course_do_id)`

- 用途：提交选课请求
- 参数来源：
  - `course_id`: 来自 `Course.kch_id`
  - `course_do_id`: 来自选定 `Jxb.do_id`

::: code-group

```rs [Rust]
client.select_course(&course.kch_id, &course.jxb[0].do_id)?;
```

```py [Python]
client.select_course(course.kch_id, course.jxb[0].do_id)
```

:::

### `ver()`

- 用途：获取教务系统前端版本号
- 返回：`Option<String>` — 版本号，未获取到时为 `None`

::: code-group

```rs [Rust]
let ver = client.ver()?;
```

```py [Python]
ver = client.ver()
```

:::

### `insert_cookie(cookie)`

- 用途：向会话中插入**单条** Cookie，便于调试或接管会话
- 参数：
  - `cookie`: Cookie 字符串，整体作为一条 `Set-Cookie` 解析；分号后的内容被视为 Cookie 属性（如 `Path`、`HttpOnly`），而非独立的 Cookie
- Feature：仅在启用 `reqwest_cookie_store` 时可用

::: code-group

```rs [Rust]
client.insert_cookie("JSESSIONID=xxxx")?;
client.insert_cookie("zstack_cookie=YYY")?;
```

```py [Python]
client.insert_cookie("JSESSIONID=xxxx")
client.insert_cookie("zstack_cookie=YYY")
```

:::

### `insert_cookies(cookies)`

- 用途：一次性插入**多条** Cookie，便于调试或接管会话
- 参数：
  - `cookies`: 以 `;` 分隔的多条 Cookie 字符串，每段作为独立 Cookie 分别插入
- Feature：仅在启用 `reqwest_cookie_store` 时可用

::: tip insert_cookie vs insert_cookies

- `insert_cookie` 将整个字符串按 `Set-Cookie` 头格式解析，只有第一个 `name=value` 对会被写入，分号之后的部分被当作 Cookie 属性（`Path`、`Domain`、`HttpOnly` 等）忽略。适合插入单条 Cookie 或带属性的完整 `Set-Cookie` 值。
- `insert_cookies` 先按 `;` 拆分字符串，再逐段调用 `insert_cookie`。适合一次性写入多条 Cookie，例如浏览器 `document.cookie` 或请求头 `Cookie` 的格式：`name1=value1; name2=value2`。
  :::

::: code-group

```rs [Rust]
client.insert_cookies("JSESSIONID=xxxx; X-LB=yyyy")?;
client.insert_cookies("JSESSIONID=xxxx; zstack_cookie=YYY")?;
```

```py [Python]
client.insert_cookies("JSESSIONID=xxxx; X-LB=yyyy")
client.insert_cookies("JSESSIONID=xxxx; zstack_cookie=YYY")
```

:::

### `clear_cookie()`

- 用途：清除所有 Cookie，恢复默认会话行为
- Feature：仅在启用 `reqwest_cookie_store` 时可用

::: code-group

```rs [Rust]
client.clear_cookie();
```

```py [Python]
client.clear_cookie()
```

:::

### `cookies()`

- 用途：获取当前会话中的关键 Cookie 字符串
- 返回：`Option<String>` — Cookie 字符串，无 Cookie 时为 `None`
- Feature：仅在启用 `reqwest_cookie_store` 时可用
- 仅在 Rust async `Client` 中可用

::: code-group

```rs [Rust]
let cookies: Option<String> = client.cookies();
```

:::

### `set_converter(converter)`

- 用途：设置 URL 转换器，用于 WebVPN 等场景
- Feature：仅在启用 `converter` 时可用

::: code-group

```rs [Rust]
use lnu_elytra::{Client, csvpn_converter};
let client = Client::new().set_converter(csvpn_converter);
```

:::

## `Course` 方法

### `try_select_0(client)`

- 用途：按默认策略（第一个教学班）尝试选课
- 参数来源：
  - `client`: 已登录且已初始化的 `Client` 实例

`course_id`: 内部取自 `self.kch_id`
`course_do_id`: 内部取自 `self.jxb[0].do_id`

::: code-group

```rs [Rust]
course.try_select_0_blocking(&client)?;
```

```py [Python]
course.try_select_0(client)
```

:::

### `try_select_by_time(client, q)`

- 用途：按上课时间条件筛选教学班后尝试选课
- 参数来源：
  - `client`: 已登录且已初始化的 `Client` 实例
  - `q`: 用户输入的时间匹配条件（例如 `星期四第9-10节`）

`course_id`: 内部取自 `self.kch_id`
`course_do_id`: 内部取自 `self.jxb` 中 `sksj` 包含 `q` 的第一个教学班的 `do_id`

::: code-group

```rs [Rust]
course.try_select_by_time_blocking(&client, "星期四第9-10节")?;
```

```py [Python]
course.try_select_by_time(client, "星期四第9-10节")
```

:::

## 数据结构

### `Course`

| 字段      | 类型       | 说明        |
| --------- | ---------- | ----------- |
| `xkkz_id` | `String`   | 选课控制 ID |
| `kch_id`  | `String`   | 课程号 ID   |
| `jxb`     | `Vec<Jxb>` | 教学班列表  |

### `Jxb`

| 字段     | 类型     | 说明                                  |
| -------- | -------- | ------------------------------------- |
| `jxb_id` | `String` | 教学班 ID                             |
| `do_id`  | `String` | 执行 ID（用于选课）                   |
| `jsxx`   | `String` | 教师信息                              |
| `sksj`   | `String` | 上课时间，如 `星期四第9-10节{9-16周}` |

### `SelectCourseResponse`

- 用途：承载选课请求结果

| 字段/方法      | 类型             | 说明                           |
| -------------- | ---------------- | ------------------------------ |
| `flag`         | `String`         | `"1"` 表示成功，`"0"` 表示失败 |
| `msg`          | `Option<String>` | 错误信息                       |
| `is_success()` | `bool`           | 判断是否选课成功               |
| `msg()`        | `Option<&str>`   | 获取错误信息                   |

来自选课接口响应 JSON，常见返回示例：

```text
{ flag: "1", msg: None }
{ flag: "0", msg: Some("对不起，当前未开放选课！") }
{ flag: "0", msg: Some("选课频率过高，请稍后重试！") }
{ flag: "0", msg: Some("一门课程只能选一个教学班，不可再选！") }
{ flag: "0", msg: Some("超过体育分项本学期本专业最高选课门次限制，不可选！") }
{ flag: "0", msg: Some("超过通识选修课本学期本专业最高选课门次限制，不可选！") }
```

## Python `AsyncClient`

异步版本的 `Client`，所有网络方法返回 `Awaitable`，适用于 `asyncio` 场景。

```py
from lnu_elytra import AsyncClient

client = AsyncClient()
# client = AsyncClient(base="http://jwxt.gcc.edu.cn")

await client.login("账号", "密码")
await client.init()
course = await client.fetch_courses("(2025-2026-2)-77101504-02")
result = await client.select_course(course.kch_id, course.jxb[0].do_id)
```

`Course` 对应的异步方法：

```py
await course.async_try_select_0(client)
await course.async_try_select_by_time(client, "星期四第9-10节")
```
