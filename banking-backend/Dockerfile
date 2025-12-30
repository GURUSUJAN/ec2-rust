FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /app/target/release/banking-backend .
EXPOSE 8080
CMD ["./banking-backend"]
