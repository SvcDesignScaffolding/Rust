
# 概述
这个项目是一个使用 Actix-web 框架搭建的简单 HTTP 服务器，提供两个 API：查询 (/api/query) 和插入 (/api/insert)。同时，它包含了跨域资源共享 (CORS) 的支持。

# 技术框架 

- Actix-web: 异步 Web 框架，用于构建高性能和可伸缩的 Web 应用,支持异步请求处理和并发处理。
- Actix-cors: 中间件，用于添加跨域资源共享 (CORS) 支持。允许前端从不同的源（域、协议或端口）请求资源。
- Serde: 序列化/反序列化库，用于将结构体转换为 JSON 格式。

# 代码结构

```
my_rust_server
|-- src
|   |-- lib.rs
|   |-- main.rs
|-- tests
|   |-- integration_test.rs
```

- Response 结构体： 表示 API 响应的数据结构，包含一个字符串字段 message。
- query_handler 函数：
  * 处理查询请求 (/api/query)。 返回 JSON 格式的响应，表示查询成功。
- insert_handler 函数：
- 处理插入请求 (/api/insert)。
返回 JSON 格式的响应，表示插入成功。

- run_server 函数：

创建 Actix-web 应用和 HTTP 服务器。
使用 Actix-cors 中间件配置跨域支持。
定义两个路由，分别映射到 query_handler 和 insert_handler。
绑定服务器地址为 127.0.0.1:8080。
启动服务器。
运行项目
添加依赖：

在 Cargo.toml 中添加依赖：actix-web, actix-cors, 和 serde。
执行 cargo run 命令：

运行 cargo run 命令启动服务器。
服务器将在 127.0.0.1:8080 上监听请求。
测试 API：

使用工具如 curl 或浏览器，访问 http://127.0.0.1:8080/api/query 和 http://127.0.0.1:8080/api/insert，验证 API 的功能和跨域支持。

- 编译项目 cargo build
- 运行项目 cargo run
- 运行测试 cargo test

测试命令

- 查询 API 测试 curl http://localhost:80/api/query
- 插入 API 测试 curl -X POST http://localhost:80/api/insert

