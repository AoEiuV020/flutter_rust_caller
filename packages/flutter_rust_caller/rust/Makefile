PROJECT_NAME := flutter_rust_caller
PREBUILD_BASE := ../prebuild

# 构建模式设置，默认为release模式
BUILD_MODE ?= release

# 根据构建模式设置 Rust 构建参数
ifeq ($(BUILD_MODE),debug)
	CARGO_FLAGS :=
	CARGO_PROFILE := debug
else
	CARGO_FLAGS := --release
	CARGO_PROFILE := release
endif

# 检测操作系统类型
ifeq ($(OS),Windows_NT)
	UNAME_S := Windows_NT
	ANDROID_HOME ?= $(LOCALAPPDATA)/Android/Sdk
	NDK_PLATFORM := windows-x86_64
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		ANDROID_HOME ?= $(HOME)/Android/Sdk
		NDK_PLATFORM := linux-x86_64
	else ifeq ($(UNAME_S),Darwin)
		ANDROID_HOME ?= $(HOME)/Library/Android/sdk
		NDK_PLATFORM := darwin-x86_64
	endif
endif

NDK_VERSION ?= 27.0.12077973
NDK_BIN ?= $(subst \,/,$(ANDROID_HOME))/ndk/$(NDK_VERSION)/toolchains/llvm/prebuilt/$(NDK_PLATFORM)/bin
NDK_EXISTS := $(if $(wildcard $(NDK_BIN)),true,false)

# 根据目标名称设置平台路径
LIB_PREFIX = lib
ifeq ($(findstring ios,$(MAKECMDGOALS)),ios)
	PLATFORM = iOS
else ifeq ($(findstring android,$(MAKECMDGOALS)),android)
	PLATFORM = Android
else ifeq ($(findstring macos,$(MAKECMDGOALS)),macos)
	PLATFORM = macOS
else ifeq ($(MAKECMDGOALS),windows)
	PLATFORM = Windows
	LIB_PREFIX :=
else ifeq ($(findstring linux,$(MAKECMDGOALS)),linux)
	PLATFORM = Linux
else ifeq ($(MAKECMDGOALS),web)
	PLATFORM = Web
else ifeq ($(MAKECMDGOALS),all)
	# 设置默认平台，将在 all 目标中处理
else ifeq ($(MAKECMDGOALS),)
	# 如果没有指定目标，也使用默认平台
else
	PLATFORM = $(MAKECMDGOALS)
endif

PREBUILD_PATH = $(PREBUILD_BASE)/$(PLATFORM)
LIB_NAME := $(LIB_PREFIX)$(PROJECT_NAME)

export PREBUILD_PATH
export LIB_NAME
export CARGO_FLAGS
export BUILD_MODE
export PROJECT_NAME
export CARGO_PROFILE

# 添加默认目标，根据系统类型自动选择目标
.PHONY: all debug release
.PHONY: android android-armv7a android-arm64 android-x86 android-x86_64
.PHONY: ios ios-arm64 ios-x86_64-sim ios-arm64-sim
.PHONY: macos macos-arm64 macos-amd64 macos-universal
.PHONY: windows linux linux-amd64 linux-arm64 web clean

all:
ifeq ($(OS),Windows_NT)
	$(MAKE) windows
else ifeq ($(UNAME_S),Linux)
	$(MAKE) linux
else ifeq ($(UNAME_S),Darwin)
	$(MAKE) macos
	# 在macOS上追加执行ios任务
	$(MAKE) ios
else
	$(error Unsupported OS for default build: $(UNAME_S))
endif
	# 如果NDK存在，追加执行android任务（所有平台通用）
	@if [ "$(NDK_EXISTS)" = "true" ]; then \
		$(MAKE) android; \
	fi
	# 追加执行web任务（所有平台通用）
	$(MAKE) web

# 添加debug目标
debug:
	$(MAKE) BUILD_MODE=debug all

# 添加release目标
release:
	$(MAKE) BUILD_MODE=release all

# 设置 all 为默认目标
.DEFAULT_GOAL := all

# ============== Android Targets ==============

android-armv7a: CURRENT_ARCH := armeabi-v7a
android-armv7a:
	mkdir -p "$(PREBUILD_PATH)/$(CURRENT_ARCH)"
	cargo build $(CARGO_FLAGS) --target armv7-linux-androideabi
	cp "target/armv7-linux-androideabi/$(CARGO_PROFILE)/libflutter_rust_caller.so" "$(PREBUILD_PATH)/$(CURRENT_ARCH)/${LIB_NAME}.so"

android-arm64: CURRENT_ARCH := arm64-v8a
android-arm64:
	mkdir -p "$(PREBUILD_PATH)/$(CURRENT_ARCH)"
	cargo build $(CARGO_FLAGS) --target aarch64-linux-android
	cp "target/aarch64-linux-android/$(CARGO_PROFILE)/libflutter_rust_caller.so" "$(PREBUILD_PATH)/$(CURRENT_ARCH)/${LIB_NAME}.so"

android-x86: CURRENT_ARCH := x86
android-x86:
	mkdir -p "$(PREBUILD_PATH)/$(CURRENT_ARCH)"
	cargo build $(CARGO_FLAGS) --target i686-linux-android
	cp "target/i686-linux-android/$(CARGO_PROFILE)/libflutter_rust_caller.so" "$(PREBUILD_PATH)/$(CURRENT_ARCH)/${LIB_NAME}.so"

android-x86_64: CURRENT_ARCH := x86_64
android-x86_64:
	mkdir -p "$(PREBUILD_PATH)/$(CURRENT_ARCH)"
	cargo build $(CARGO_FLAGS) --target x86_64-linux-android
	cp "target/x86_64-linux-android/$(CARGO_PROFILE)/libflutter_rust_caller.so" "$(PREBUILD_PATH)/$(CURRENT_ARCH)/${LIB_NAME}.so"

android: android-armv7a android-arm64 android-x86 android-x86_64

# ============== iOS Targets ==============

ios-x86_64-sim:
	mkdir -p "$(PREBUILD_PATH)/x86_64"
	cargo build $(CARGO_FLAGS) --target x86_64-apple-ios
	cp "target/x86_64-apple-ios/$(CARGO_PROFILE)/libflutter_rust_caller.a" "$(PREBUILD_PATH)/x86_64/${LIB_NAME}.a"

ios-arm64-sim:
	mkdir -p "$(PREBUILD_PATH)/arm64"
	cargo build $(CARGO_FLAGS) --target aarch64-apple-ios-sim
	cp "target/aarch64-apple-ios-sim/$(CARGO_PROFILE)/libflutter_rust_caller.a" "$(PREBUILD_PATH)/arm64/${LIB_NAME}.a"

ios-arm64:
	mkdir -p "$(PREBUILD_PATH)/arm64"
	cargo build $(CARGO_FLAGS) --target aarch64-apple-ios
	cp "target/aarch64-apple-ios/$(CARGO_PROFILE)/libflutter_rust_caller.a" "$(PREBUILD_PATH)/arm64/${LIB_NAME}.a"

ios: ios-x86_64-sim ios-arm64-sim ios-arm64

# ============== macOS Targets ==============

macos-arm64:
	mkdir -p "$(PREBUILD_PATH)/arm64"
	cargo build $(CARGO_FLAGS) --target aarch64-apple-darwin
	cp "target/aarch64-apple-darwin/$(CARGO_PROFILE)/libflutter_rust_caller.a" "$(PREBUILD_PATH)/arm64/${LIB_NAME}.a"

macos-amd64:
	mkdir -p "$(PREBUILD_PATH)/x86_64"
	cargo build $(CARGO_FLAGS) --target x86_64-apple-darwin
	cp "target/x86_64-apple-darwin/$(CARGO_PROFILE)/libflutter_rust_caller.a" "$(PREBUILD_PATH)/x86_64/${LIB_NAME}.a"

macos-universal: macos-arm64 macos-amd64
	mkdir -p "$(PREBUILD_PATH)/universal/"
	lipo \
		-create \
		"$(PREBUILD_PATH)/arm64/${LIB_NAME}.a" \
		"$(PREBUILD_PATH)/x86_64/${LIB_NAME}.a" \
		-output "$(PREBUILD_PATH)/universal/${LIB_NAME}.a"
	rm "$(PREBUILD_PATH)/arm64/${LIB_NAME}.a"
	rm "$(PREBUILD_PATH)/x86_64/${LIB_NAME}.a"

macos: macos-arm64 macos-amd64

# ============== Windows Targets ==============

windows:
	mkdir -p "$(PREBUILD_PATH)/AMD64"
	cargo build $(CARGO_FLAGS) --target x86_64-pc-windows-gnu
	cp "target/x86_64-pc-windows-gnu/$(CARGO_PROFILE)/libflutter_rust_caller.dll" "$(PREBUILD_PATH)/AMD64/${LIB_NAME}.dll"

# ============== Linux Targets ==============

linux-amd64:
	mkdir -p "$(PREBUILD_PATH)/x86_64"
	cargo build $(CARGO_FLAGS) --target x86_64-unknown-linux-gnu
	cp "target/x86_64-unknown-linux-gnu/$(CARGO_PROFILE)/libflutter_rust_caller.so" "$(PREBUILD_PATH)/x86_64/${LIB_NAME}.so"

linux-arm64:
	mkdir -p "$(PREBUILD_PATH)/aarch64"
	cargo build $(CARGO_FLAGS) --target aarch64-unknown-linux-gnu
	cp "target/aarch64-unknown-linux-gnu/$(CARGO_PROFILE)/libflutter_rust_caller.so" "$(PREBUILD_PATH)/aarch64/${LIB_NAME}.so"

linux:
	@ARCH=$$(uname -m); \
	if [ "$$ARCH" = "x86_64" ]; then \
		echo "检测到 x86_64 架构，执行 linux-amd64 构建"; \
		$(MAKE) linux-amd64; \
	elif [ "$$ARCH" = "aarch64" ] || [ "$$ARCH" = "arm64" ]; then \
		echo "检测到 ARM64 架构，执行 linux-arm64 构建"; \
		$(MAKE) linux-arm64; \
	else \
		echo "警告: 未识别的架构 $$ARCH，默认执行 linux-amd64 构建"; \
		$(MAKE) linux-amd64; \
	fi

# ============== Web Target ==============

web:
	mkdir -p "$(PREBUILD_PATH)"
	cargo build $(CARGO_FLAGS) --target wasm32-unknown-unknown
	cp "target/wasm32-unknown-unknown/$(CARGO_PROFILE)/flutter_rust_caller.wasm" "$(PREBUILD_PATH)/${LIB_NAME}.wasm"

# ============== Cleanup ==============

clean:
	cargo clean
	rm -rf $(PREBUILD_BASE)
