FROM alpine/git:2.36.2 as download

RUN git clone https://github.com/qooba/anonymize-rs /anonymize-rs

FROM rust:1.70.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new anonymize-rs
WORKDIR /usr/src/anonymize-rs
COPY --from=download /anonymize-rs/anonymize-rs/Cargo.toml .

RUN apt update && apt install musl-tools musl-dev -yq
RUN cargo build --release

COPY --from=download /anonymize-rs/anonymize-rs/src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .
#RUN cargo install --path .

# Bundle Stage
FROM scratch
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/anonymize_rs /usr/local/bin/anonymize_rs
USER 1000
ENTRYPOINT ["anonymize_rs"]
