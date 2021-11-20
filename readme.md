# Rust-koan

## Setup
### install

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run in docker

```shell
# speed up if needed
# export RUSTUP_DIST_SERVER=https://mirrors.sjtug.sjtu.edu.cn/rust-static
# export RUSTUP_UPDATE_ROOT=https://mirrors.sjtug.sjtu.edu.cn/rust-static/rustup

docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust cargo run

# or
SRC_DIR=scrape_url sh run.sh https://www.rust-lang.org/ rust.md
```

### tools

- rust-analyzer
- rust syntax
- better toml
- rust test lens
- Tabnine
