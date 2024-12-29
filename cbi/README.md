<div align="center">
<h1>Crowbar Info</h1>
</div>
<div align="center">

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

## 安装

目前仅支持 Windows 操作系统。

```sh
git clone https://github.com/Rosmeowtis/crowbar-info.git
cd crowbar-info/cbi
cargo install --path .
```

这会使用你本机上的 Rust 编译工具链编译出 cbi.exe，并安装在 `$HOME/.cargo/bin` 目录下。