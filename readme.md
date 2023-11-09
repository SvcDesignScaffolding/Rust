
该示例代码使用了 actix-web 框架来实现一个简单的 HTTP 服务器。该服务器提供了三个路由：

/logql：用于执行 LogQL 查询
/promql：用于执行 promQL 查询
/tracing：用于查询 trace
每个路由都使用 web::Query 解析请求参数，然后使用 clickhouse 库来执行查询。查询结果使用 serde_json 库进行序列化，以便以 JSON 格式返回给客户端。

serde_json 库提供了 to_string() 和 to_string_pretty() 方法来序列化数据。to_string() 方法将数据序列化为 JSON 字符串，而 to_string_pretty() 方法将数据序列化为格式化后的 JSON 字符串。
