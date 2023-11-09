#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logql_handler() {
        // 创建一个 ClickHouse 客户端
        let client = Client::new("localhost:9000");

        // 构造一个查询字符串
        let query = "SELECT * FROM logs WHERE level = 'INFO'";

        // 执行查询
        let rows = client.query(query.as_str()).await.unwrap();

        // 构造期望的响应
        let expected_response = serde_json::to_string_pretty(&rows).unwrap();

        // 调用 `logql_handler()` 函数
        let response = logql_handler().await;

        // 断言响应与期望的响应相等
        assert_eq!(response.body(), expected_response);
    }

    #[test]
    fn test_promql_handler() {
        // 创建一个 ClickHouse 客户端
        let client = Client::new("localhost:9000");

        // 构造一个查询字符串
        let query = "up{job=\"my-job\"}";

        // 执行查询
        let rows = client.query(query.as_str(), &"PromQL").await.unwrap();

        // 构造期望的响应
        let expected_response = serde_json::to_string_pretty(&rows).unwrap();

        // 调用 `promql_handler()` 函数
        let response = promql_handler().await;

        // 断言响应与期望的响应相等
        assert_eq!(response.body(), expected_response);
    }

    #[test]
    fn test_tracing_handler() {
        // 创建一个 ClickHouse 客户端
        let client = Client::new("localhost:9000");

        // 构造一个 trace ID
        let trace_id = "1234567890abcdef";

        // 查询 trace
        let rows = client.query(
            "SELECT * FROM tracing WHERE trace_id = ?",
            &[trace_id],
        ).await.unwrap();

        // 构造期望的响应
        let expected_response = serde_json::to_string_pretty(&rows).unwrap();

        // 调用 `tracing_handler()` 函数
        let response = tracing_handler().await;

        // 断言响应与期望的响应相等
        assert_eq!(response.body(), expected_response);
    }
}
