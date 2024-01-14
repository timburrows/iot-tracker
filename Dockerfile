FROM rust:latest AS builder
RUN update-ca-certificates

WORKDIR /iot

COPY ./ .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /iot

# Copy the build
COPY --from=builder /iot/target/release/iot_tracker ./

CMD ["/iot_tracker/iot_tracker"]