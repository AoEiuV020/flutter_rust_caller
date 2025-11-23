#!/bin/bash
set -e

cd /workspace

cd packages/flutter_rust_caller/rust
make linux

# 执行传入的命令或者启动 bash
exec "$@"
