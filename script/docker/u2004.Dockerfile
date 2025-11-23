# 使用 Ubuntu 20.04 作为基础镜像
FROM ubuntu:20.04

# 避免交互式提示
ENV DEBIAN_FRONTEND=noninteractive

# 设置时区
ENV TZ=Asia/Shanghai

# 更新系统并安装基础工具
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    xz-utils \
    make \
    # Rust 开发必需
    gcc \
    g++ \
    libc6-dev \
    pkg-config \
    # 清理缓存
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# 安装 Rust
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/home/developer/.cargo
ENV PATH="/home/developer/.cargo/bin:/usr/local/rustup/bin:${PATH}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --no-modify-path --default-toolchain stable && \
    rustup target add \
        x86_64-unknown-linux-gnu \
        aarch64-unknown-linux-gnu \
        armv7-unknown-linux-gnueabihf

# 创建用户 developer (UID=1000)
RUN useradd -m -u 1000 -s /bin/bash developer \
    && echo "developer ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# 设置 Cargo 所有者
RUN chown -R developer:developer /home/developer/.cargo

# 创建工作目录
RUN mkdir -p /workspace && chown developer:developer /workspace

# 切换到 developer 用户
USER developer
WORKDIR /workspace

# 创建并设置 entrypoint 脚本
USER root
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# 切换回 developer 用户
USER developer

# 设置 entrypoint
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["echo", "构建完成"]
