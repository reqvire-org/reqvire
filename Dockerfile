FROM rust:1.86.0 AS builder

WORKDIR /usr/src/app


COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target=x86_64-unknown-linux-musl --release


FROM alpine:3.20
    
WORKDIR /app

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/reqflow .


RUN chmod +x ./reqflow

ENTRYPOINT ["/app/reqflow"]

