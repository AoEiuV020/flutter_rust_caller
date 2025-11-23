# flutter_rust_caller

Rust FFI 插件项目，用于 Flutter 跨平台开发中调用 Rust 函数

## 项目概述

本项目通过 Rust 语言实现跨平台原生功能，编译为 android/ios/windows/linux/macos/web 全平台原生库供 Flutter 调用。

## 项目结构

本项目使用以下结构：

* `rust/`: 包含 Rust 语言源代码和编译脚本，用于构建跨平台的原生库
* `lib/`: 包含插件的 Dart 代码实现，通过 `dart:ffi` 调用原生代码
* 平台文件夹 (`android`, `ios`, `windows`, `linux`, `macos`): 包含用于集成预编译库的构建文件

## 快速开始

### 环境要求

- Rust 1.81+
- Flutter 3.29.3
- Make 3.81
- Android NDK (用于编译 Android)
- Xcode (用于编译 iOS/macOS)

### 构建命令

进入 rust 项目目录，执行以下命令：

```bash
cd rust

# macOS 构建所有支持的目标（macOS, iOS, Android, Web）
make

# 单个平台构建
make android      # 所有 Android 架构
make ios          # 所有 iOS 架构
make macos        # macOS (arm64 + x86_64)
make web          # WebAssembly

# Debug 模式
make BUILD_MODE=debug
```

### Docker 编译 Linux

使用 Docker 在容器中编译 Linux 平台库：

```bash
# 在项目根目录执行，编译 Linux amd64 + arm64
./script/linux_multi_arch_docker.sh

# 或者单次编译
./script/linux_docker.sh
```

## 平台集成指南

### 通用说明

- 所有构建产物位于 `./prebuild/` 目录，按平台和架构命名放置
- 预编译库名字统一为 `libflutter_rust_caller.so/a/dll/wasm`

### Android

Android 配置已在 `android/build.gradle` 中设置，指向 prebuild 目录：

```gradle
sourceSets {
    main {
        jniLibs.srcDirs = ["../prebuild/Android"]
    }
}
```

### iOS/macOS

iOS 和 macOS 使用 CocoaPods 引入预编译库：

**重要**: 在 `ios/Classes/flutter_rust_caller.c` 和 `macos/Classes/flutter_rust_caller.c` 中，
自动生成的 `#include` 指令已被注释。这是因为 FFI 插件模板会自动生成这些文件，包含对源代码的引用。
但由于我们使用预编译库而非源代码编译，这些 include 需要注释掉。

```c
// #include "../../src/flutter_rust_caller.c"
// 注意: 已使用预编译库，无需编译源代码
```

### Windows/Linux

Windows 和 Linux 使用 CMake 配置，指向 prebuild 目录中的预编译库。

## 编译配置详解

### Cargo 配置 (.cargo/config.toml)

为了在 macOS 上交叉编译 Android，配置使用 NDK 的 clang 作为链接器：

```toml
[target.armv7-linux-androideabi]
linker = "/path/to/Android/sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"
```

这允许在 macOS 上直接编译 Android 库，无需在 Linux 环境编译。

### Makefile 变量

- `PROJECT_NAME`: 库名称（flutter_rust_caller）
- `BUILD_MODE`: 构建模式（debug/release，默认 release）
- `CARGO_FLAGS`: Cargo 构建标志
- `CARGO_PROFILE`: Cargo 输出目录名（debug 或 release）

## 自定义开发

### 修改 Rust 代码

编辑 `rust/src/` 目录下的文件：

- `business.rs`: 核心业务逻辑实现
- `call.rs`: JSON 参数解析和方法分派
- `ffi.rs`: C FFI 导出函数

修改后重新执行 `make` 命令重新编译。

### 添加新的 Rust 函数

1. 在 `business.rs` 中实现业务逻辑
2. 在 `call.rs` 中的 `execute` 函数中添加方法分派
3. 重新编译：`make`

## 故障排除

### Android NDK 路径错误

如果 Android 编译失败，检查 `.cargo/config.toml` 中的 NDK 路径。获取正确路径：

```bash
echo $ANDROID_HOME/ndk/$(ls $ANDROID_HOME/ndk | head -1)/toolchains/llvm/prebuilt/darwin-x86_64/bin
```

### Docker 网络问题

如果 Docker 编译超时，尝试配置 DNS 或更换网络。

## 更多信息

- [Rust 文档](https://doc.rust-lang.org/)
- [Cargo 配置](https://doc.rust-lang.org/cargo/)
- [Flutter FFI 文档](https://flutter.dev/docs/development/platform-integration/c-interop)

