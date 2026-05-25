from lnu_elytra.tracing import Tracing, GlobalTracingConfig, SimpleConfig, subscriber, layers
from lnu_elytra import Client;
import asyncio

cfg = GlobalTracingConfig(
    SimpleConfig(
        subscriber.Config(
            layers.file.Config(
                file_path=None,
                pretty=True,
                filter="info",
                json=False,
            )
        )
    )
)

def main():
    client = Client()

    u = input("请输入用户名：")
    p = input("请输入密码：")

    tgs = input("请输入教学班（多个用逗号分割）：")
    tgs = tgs.split(",")

    try:
        client.login(u, p)
    except Exception as e:
        print(f"登陆失败，请检查用户名或者密码是否正确：{e}")
        return

    while True:

        try:
            client.init()
        except Exception as e:
            print(f"初始化失败，正在重试：{e}")
            asyncio.sleep(1)
            continue
        print("初始化成功")
        break

    for tg in tgs:
        while True:
            try:
                result = client.select_course(tg)
                if result.flag == "1":
                    print(f"选课成功：{result.message}")
                else:
                    print(f"选课失败：{result.message}")
                    continue
            except Exception as e:
                print(f"选课失败：{e}")
                asyncio.sleep(1)
                continue
            break



if __name__ == "__main__":
    async def async_main():
        async with Tracing(cfg):
            main()
            pass
    asyncio.run(async_main())