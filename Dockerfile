FROM rust
WORKDIR /app
RUN echo "fn main() {}" > dummy.rs
ARG SRC_DIR
ENV SRC_DIR=${SRC_DIR:-}
COPY ${SRC_DIR}/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY ${SRC_DIR}/. .
RUN cargo build --release
ENTRYPOINT ["target/release/app"]