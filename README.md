<div align="center">
<h1>Crowbar Info</h1>
</div>
<div align="center">

<div align="center">
Crowbar Info 是一个用于查询支持 [Source Server Query Protocol](https://developer.valvesoftware.com/wiki/Server_queries) 的游戏服务器信息的工具，其支持查询的服务器类型包括但不限于 L4D2 等。
</div>

## 目录

+ [cbi 命令行工具](./cbi/)

## 功能特性

1. 提供命令行工具 crowbar-info （简写为 cbi），可查询或批量查询指定地址的服务器信息；
2. 提供后台应用 crowbar-info-json （简写为 cbis），可从 stdin 读取JSON 格式的查询指令，并向 stdout 返回 JSON 格式结果；
