import json
import subprocess as s
from pprint import pprint

# 先 cargo install --path cbi 或者手动将编译好的 cbi 可执行文件放在 PATH 目录下
# 将 l4d2.server 替换成某服务器的域名或IP，并修改端口号范围即可测试
# localhost 是在本地未提供服务的情况下查看错误输出格式的

if __name__ == "__main__":
    with s.Popen(["cbi", "json"], stdin=s.PIPE, stdout=s.PIPE) as proc:
        # 注意，需要手动发送换行符并 flush 缓冲区，否则容易死锁
        arg = {
            "cmd": "full",
            "hosts": [f"l4d2.server:{i}" for i in range(28001, 28003)] + ["localhost:27015"],
        }
        proc.stdin.write(json.dumps(arg).encode() + b"\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        output = line.strip().decode()

        # 将返回 JSON 格式的数据，可由 Python 处理
        pprint(json.loads(output))

        arg = {
            "cmd": "info",
            "hosts": [f"l4d2.server:{i}" for i in range(28004, 28006)] + ["jdklfas///91**不是域名"],
        }
        proc.stdin.write(json.dumps(arg).encode() + b"\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        output = line.strip().decode()

        # 将返回 JSON 格式的数据，可由 Python 处理
        pprint(json.loads(output))


        proc.stdin.close()
        proc.terminate()
        proc.wait(0.2)
