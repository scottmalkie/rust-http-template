FROM openfaas/of-watchdog:0.8.0 as watchdog
FROM arm64v8/rust:1.45-slim-stretch as builder

# Set working directory
WORKDIR /home/rust

# Copy all the sources
COPY main ./main

# Compile binary
RUN rustup target add aarch64-unknown-linux-musl
RUN cd main && cargo build --target aarch64-unknown-linux-musl --release

# Build actual image
FROM scratch

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /fwatchdog

# Copy rust binary
COPY --from=builder /home/rust/main/target/aarch64-unknown-linux-musl/release/main /main

# Set up watchdog for HTTP mode
ENV fprocess="/main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

CMD ["/fwatchdog"]
