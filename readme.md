# Rust-koan

> 记录[《陈天 · Rust 编程第一课》](https://time.geekbang.org/column/intro/100085301?code=giOSIlBIc9I7DCzTSPw4H4tKiHXRnI4L6GCNKmz-XnI%3D&source=app_share)的学习之路.
> [课程仓库](https://github.com/tyrchen/geektime-rust) 

## Setup

### vscoode tools

- rust-analyzer
- rust syntax
- better toml
- rust test lens
- Tabnine

### Run in local

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# inside src directory
# 国内加速如果需要的话
# export RUSTUP_DIST_SERVER=https://mirrors.sjtug.sjtu.edu.cn/rust-static
# export RUSTUP_UPDATE_ROOT=https://mirrors.sjtug.sjtu.edu.cn/rust-static/rustup

cargo run
```

### Run in docker

```shell
# 使用docker构建
docker run -it --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust cargo run

# 使用docker缓存依赖版
SRC_DIR=scrape_url sh run.sh https://www.rust-lang.org/ rust.md
```

#### 添加依赖

使用`cargo-edit`，类似`yarn`

```shell
cargo install cargo-edit
cargo add anyhow colored jsonxf mime
cargo add clap --allow-prerelease
cargo add reqwest --features "json rustls-tls"
cargo add tokio --features full
```
