FROM rust:1.68.0-slim as builder

WORKDIR /usr/src/

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo new --bin dummy && \
    cd dummy && \
    echo 'log = "0.4"' >> Cargo.toml && \
    cargo build

COPY . /usr/src

ARG APP=restserver
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cp ./target/x86_64-unknown-linux-musl/release/${APP} ./app

FROM scratch

EXPOSE 8080
ENV RUST_LOG=debug

COPY --from=builder /usr/src/app ./

CMD ["/app"]
