<div align="center">
<h1>Crowbar Info</h1>
</div>

<div align="center">
Crowbar Info 是一个用于查询支持 <a href="https://developer.valvesoftware.com/wiki/Server_queries">Source Server Query Protocol</a> 的游戏服务器信息的工具，其支持查询的服务器类型包括但不限于 L4D2 等。
</div>

## 使用

```sh
cbi full <host>:<port>

╭──────────────────────────────────────────────────────────╮
│ <服务器名称>           <地图代码>  <当前玩家> <最大玩家>    │
│──────────────────────────────────────────────────────────│
│   0  <玩家名1>             <玩家分数>       <游玩时间>     │
│   1  <玩家名2>             <玩家分数>       <游玩时间>     │
│   2  <玩家名3>             <玩家分数>       <游玩时间>     │
│   3  <玩家名4>             <玩家分数>       <游玩时间>     │
│   4  <玩家名5>             <玩家分数>       <游玩时间>     │
│   5  <玩家名6>             <玩家分数>       <游玩时间>     │
╰──────────────────────────────────────────────────────────╯
```

也可以同时查询多个服务器的信息：

```sh
cbi full <host1>:<port1> <host2>:<port2> <host3>:<port3> ...
```

如果只想查询对应服务器的基本信息，则：

```sh
cbi info <host>:<port>
```

如果只想查询对应服务器的玩家列表，则：

```sh
cbi players <host>:<port>
```

### json 接口

可以通过管道与其他程序交互，其输入输出示例为：

```sh
$ cbi json

{"cmd": "info", "hosts": ["example.com:27015"]}          # 输入
{"cmd":"info","payloads":[{"Info":{"host":"example.com:27015","info":{"Ok":{"name":"示例服务器","map":"c2m1_highway","players":15,"max_players":16,"game":"Left 4 Dead 2","app_id":550,"bots":0,"visibility":false,"vac":true,"version":"2.2.4.3"}}}}]}

{"cmd": "players", "hosts": ["example.com:27015"]}       # 输入
{"cmd":"players","payloads":[{"Players":{"host":"example.com:27015","players":{"Ok":[{"name":"Miya 17","score":114,"duration":18831.186},{"name":"62b48cb6","score":6,"duration":9212.259},{"name":"几把小狗","score":65,"duration":8480.743},{"name":"AradiaUwU","score":9,"duration":8281.043},{"name":"Dlclove717","score":4,"duration":4332.616},{"name":"离歌歌","score":11,"duration":4033.3323},{"name":"糖六颗","score":25,"duration":3229.3892},{"name":"PnFs","score":29,"duration":3143.2146},{"name":"promised_sea","score":13,"duration":1642.1367},{"name":"不爱笑的猫","score":12,"duration":951.1873},{"name":"椎兔","score":16,"duration":564.4128},{"name":"木白","score":7,"duration":492.46066},{"name":"红果","score":8,"duration":366.21103},{"name":"靓坤","score":1,"duration":229.23555},{"name":"Spicy Dan","score":1,"duration":43.925163}]}}}]}

{"cmd":"full","hosts":["example.com:27015"]}             # 输入
{"cmd":"full","payloads":[{"Full":{"host":"example.com:27015","info":{"Ok":{"name":"示例服务器","map":"c2m1_highway","players":15,"max_players":16,"game":"Left 4 Dead 2","app_id":550,"bots":0,"visibility":false,"vac":true,"version":"2.2.4.3"}},"players":{"Ok"":[{"name":"Miya 17","score":121,"duration":18847.336},{"name":"62b48cb6","score":6,"duration":9228.409},{"name":"几把小狗","score":69,"duration":8496.893},{"name":"AradiaUwU","score":9,"duration":8297.193},{"name":"Dlclove717","score":5,"duration":4348.766},{"name":" 离歌歌","score":11,"duration":4049.4824},{"name":"糖六颗","score":26,"duration":3245.539},{"name":"PnFs","score":29,"duration":3159.3645},{"name":"promised_sea","score":13,"duration":1658.2867},{"name":"不爱笑的猫","score":12,"duration":967.3373},{"name":"椎兔","score":18,"duration":580.56274},{"name":"木白","score":7,"duration":508.61063},{"name":"红果","score":8,"duration":382.36102},{"name":"靓坤","score":2,"duration":245.38553},{"name":"Spicy Dan","score":2,"duration":60.075134}]}}}]}
```

Python subprocess 示例

```python
import json
import subprocess as s
from pprint import pprint

# 先 cargo install cbi 或者手动将编译好的 cbi 可执行文件放在 PATH 目录下
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
```

#### JSON 格式

输入格式（以 TypeScript 为例）：

```typescript
interface Input {
    cmd: "full" | "info" | "players";
    hosts: string[];
}
```

输出格式：

```typescript
interface Output {
    cmd: "full" | "info" | "players";
    payloads: Payload[];
}

interface Payload {
    Info?: {
        host: string,
        info: {
            Ok?: A2SInfo,
            Err?: string, // 错误描述
        }
    };
    Players?: {
        host: string,
        players: {
            Ok?: A2SPlayer[],
            Err?: string, // 错误描述
        }
    };
    Full?: {
        host: string,
        info: {
            Ok?: A2SInfo,
            Err?: string, // 错误描述
        },
        players: {
            Ok?: A2SPlayer[]
            Err?: string, // 错误描述
        }
    };
    Err?: {
        what: {
            Err?: string, // 错误描述
        }
    }
}
```

A2SInfo 和 A2SPlayer 分别含有字段：

```rs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2SInfo {
    name: String,
    map: String,
    players: u8,
    max_players: u8,
    game: String,
    app_id: u16,
    bots: u8,
    visibility: bool,
    vac: bool,
    version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2SPlayer {
    name: String,
    score: i32,
    duration: f32,
}
```

## 安装

目前仅支持 Windows 操作系统。（Linux 下未测试）

```sh
git clone https://github.com/Rosmeowtis/crowbar-info.git
cd crowbar-info
cargo install --path cbi
```

这会使用你本机上的 Rust 编译工具链编译出 cbi.exe，并安装在 `$HOME/.cargo/bin` 目录下。