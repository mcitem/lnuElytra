# Python

[pypi.org/lnu_elytra](https://pypi.org/project/lnu_elytra)

## 安装

```sh
pip install lnu_elytra
```

## 使用

```py
from lnu_elytra import Client

client = Client()
client.login("账号", "密码")
client.init()
# 教学班示例：(2025-2026-2)-77101504-02
# 使用精确的教学班查询能能减少教务系统返回的数据量，有利于加快抢课。
course = client.fetch_course("教学班")
# 只有当使用精确教学班查询时，才适合直接调用 try_select_o
course.try_select_0(client)
```

## 最佳实践

例如在抢课开始前提前10分钟就登入系统，并等到抢课时间开始时在手动控制脚本继续运行（也可以通过断点调试、Cookie登录实现

```py
from lnu_elytra import Client

client = Client()
client.login("账号", "密码")

input("在抢课时间开始时回车键继续")

# 最好使用while True和try block 避免脚本被错误打断。
client.init() # 如果当前不在选课时间，init会抛出错误

while True:
    try:
        course = client.fetch_course("教学班")
        course.try_select_0(client)
        break
    except Exception as e:
        print(e)
```

可参考以下模板让python能够显示tracing提示

<<< @/../examples/python.py

效果

```
PS E:\> python .\main.py
请输入用户名：1
请输入密码：1
请输入教学班（多个用逗号分割）：1
  2026-05-25T17:56:06.681647Z  INFO lnu_elytra::method::login: 正在登录...
    at src\method\login.rs:17

  2026-05-25T17:56:07.493952Z ERROR lnu_elytra::method::login: 登录失败，未找到用户名
    at src\method\login.rs:71

登陆失败，请检查用户名或者密码是否正确：LoginFailed
```

## 异步

<<< @/../examples/python_async.py
