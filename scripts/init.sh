#!/bin/bash

# 创建目录结构
mkdir -pv src/ .github/workflows/

touch Dockerfile readme.md src/main.rs test.rs Cargo.toml 
# 提示初始化完成
echo "项目初始化完成"
