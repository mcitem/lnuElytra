# API 参考

## `Client` 方法

## `new()`

::: code-group

```rs [Rust]
let mut client = Client::new();
```

```py [Python]
client = Client()
```

:::

## `login(username, password)`

- 用途：执行登录流程
- 参数：
  - `username`: 学号
  - `password`: 密码

::: code-group

```rs [Rust]
client.login("账号", "密码");
```

```py [Python]
client.login("账号", "密码")
```

:::

## `init()`

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

## `fetch_course(q)`

- 用途：按条件查询课程并返回可操作课程对象
- 参数：
  - `q`: 课程查询关键字,建议使用精确教学班，例如：`(2025-2026-2)-77101504-02`

::: code-group

```rs [Rust]
let course = client.fetch_course("(2025-2026-2)-77101504-02")?;
```

```py [Python]
course = client.fetch_course("(2025-2026-2)-77101504-02")
```

:::

## `select_course(course_id, course_do_id)`

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

## `set_cookie_override(cookie)`

- 用途：覆盖默认 Cookie，便于调试或接管会话
- 参数：
  - `cookie`: 外部传入的完整 Cookie 字符串
- Rust Feature：仅在启用 `cookie_override` 时可用

::: code-group

```rs [Rust]
client.set_cookie_override("JSESSIONID=xxxx; X-LB=yyyy".into());
```

```py [Python]
client.set_cookie_override("JSESSIONID=xxxx; X-LB=yyyy")
```

:::

## `clear_cookie_override()`

- 用途：清除自定义 Cookie，恢复默认会话行为
- Feature：仅在启用 `cookie_override` 时可用

::: code-group

```rs [Rust]
client.clear_cookie_override();
```

```py [Python]
client.clear_cookie_override()
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

## `try_select_by_time(client, q)`

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

## `SelectCourseResponse`

- 用途：承载选课请求结果

来自选课接口响应 JSON，常见返回示例：

```text
{ flag: "1", msg: None }
{ flag: "0", msg: Some("对不起，当前未开放选课！") }
{ flag: "0", msg: Some("选课频率过高，请稍后重试！") }
{ flag: "0", msg: Some("一门课程只能选一个教学班，不可再选！") }
{ flag: "0", msg: Some("超过体育分项本学期本专业最高选课门次限制，不可选！") }
```
