#!/bin/bash
set -e

PROJECT_ROOT=$(pwd)
RUST_DIR="$PROJECT_ROOT/../../rust/rust_fileFunc"
BUILD_DIR="$PROJECT_ROOT/../../src"

echo "--- Step 1: Building Rust (GNU Target) ---"
cd $RUST_DIR
# 注意：如果用 MinGW 链接，Rust 最好生成 .a 文件
cargo build --release --target x86_64-pc-windows-gnu

echo "--- Step 2: CMake Configuration ---"
mkdir -p $BUILD_DIR
cd $BUILD_DIR
cmake -G "MinGW Makefiles" ..

echo "--- Step 3: Make ---"
cmake --build .