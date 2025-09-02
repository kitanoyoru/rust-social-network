FROM rust:1.89-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates

RUN addgroup -g 1001 -S app && \
    adduser -u 1001 -S app -G app

WORKDIR /app

COPY --from=builder /app/target/release/rust-social-network /app/rust-social-network

RUN chown app:app /app/rust-social-network

USER app

EXPOSE 3000

ENV RUST_LOG=info

CMD ["./rust-social-network", "serve", "--host", "0.0.0.0", "--port", "3000"]
