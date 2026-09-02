FROM rust:1.98.0-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/lexcheck /usr/local/bin/lexcheck
EXPOSE 3000
CMD ["lexcheck"]
