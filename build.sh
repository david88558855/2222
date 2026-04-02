#!/bin/bash

# MoonTV 编译脚本 (Linux/macOS)
# 此脚本用于在非 Windows 系统上编译 MoonTV

set -e

echo "🚀 开始编译 MoonTV..."

# 检查 Rust 是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误：未找到 Cargo，请先安装 Rust"
    echo "访问 https://rustup.rs/ 获取安装说明"
    exit 1
fi

# 显示 Rust 版本
echo "📦 Rust 版本信息:"
rustc --version
cargo --version

# 清理之前的构建（可选）
if [ "$1" == "--clean" ]; then
    echo "🧹 清理之前的构建..."
    cargo clean
fi

# 编译 Release 版本
echo "🔨 编译 Release 版本..."
cargo build --release

# 获取目标目录
TARGET_DIR="target/release"
BINARY_NAME="moontv"

# 检查编译是否成功
if [ -f "$TARGET_DIR/$BINARY_NAME" ]; then
    echo "✅ 编译成功!"
    echo "📍 二进制文件位置：$TARGET_DIR/$BINARY_NAME"
    
    # 显示文件大小
    FILE_SIZE=$(ls -lh "$TARGET_DIR/$BINARY_NAME" | awk '{print $5}')
    echo "📊 文件大小：$FILE_SIZE"
    
    # 创建运行目录结构
    DEPLOY_DIR="deploy"
    echo "📁 创建部署目录结构..."
    
    mkdir -p "$DEPLOY_DIR"
    cp "$TARGET_DIR/$BINARY_NAME" "$DEPLOY_DIR/"
    cp -r "static" "$DEPLOY_DIR/" 2>/dev/null || true
    cp "config.json" "$DEPLOY_DIR/" 2>/dev/null || true
    
    echo "✅ 部署目录已准备就绪：$DEPLOY_DIR/"
    echo ""
    echo "📋 运行说明:"
    echo "   cd $DEPLOY_DIR"
    echo "   ./$BINARY_NAME"
    echo ""
    echo "🌐 访问地址：http://localhost:8080"
else
    echo "❌ 编译失败!"
    exit 1
fi
