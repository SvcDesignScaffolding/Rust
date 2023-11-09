# 第一个阶段：构建 Rust 二进制文件
FROM rust:1.60.0-slim-buster as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# 第二个阶段：将 Rust 二进制文件复制到最终镜像
FROM scratch
COPY --from=builder /app/target/release/clickhouse-query /app
CMD ["/app/clickhouse-query"]
