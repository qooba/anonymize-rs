FROM rust:1.70.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new anonymize-rs
WORKDIR /usr/src/anonymize-rs
COPY Cargo.toml ./

RUN apt update && apt install musl-tools musl-dev -yq
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .
#RUN cargo install --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/anonymize-rs .
USER 1000
ENTRYPOINT ["./anonymize-rs"]
