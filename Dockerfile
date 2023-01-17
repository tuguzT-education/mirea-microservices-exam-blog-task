FROM rust:slim as builder
USER root
WORKDIR /usr/src/exam-task
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest as runtime
RUN apk --update --no-cache add curl
COPY --from=builder /usr/src/exam-task/target/x86_64-unknown-linux-musl/release/exam-task /usr/local/bin

ENV DATABASE_URL mongodb://localhost:27017
ENV RUST_LOG debug
EXPOSE 8080
CMD /usr/local/bin/exam-task
