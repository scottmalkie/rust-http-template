FROM openfaas/of-watchdog:0.8.0 as watchdog
FROM arm64v8/rust:1.45-slim-stretch as builder

# Set working directory
WORKDIR /home/rust

# Copy all the sources
COPY main ./main

# Compile binary
RUN rustup target add aarch64-unknown-linux-gnu
RUN cd main && cargo build --target aarch64-unknown-linux-gnu --release

# Build actual image
FROM alpine:3.12

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates \
    && addgroup -S app && adduser -S -g app app
ENV USER=app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog
RUN which fwatchdog

# Copy rust binary
COPY --from=builder /home/rust/main/target/aarch64-unknown-linux-gnu/release/main /usr/bin/main
RUN chmod +x /usr/bin/main
RUN which main

# Set up watchdog for serialzing (stdio) mode
ENV fprocess="/usr/bin/main"
ENV mode="serializing"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]