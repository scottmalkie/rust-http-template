FROM openfaas/of-watchdog:0.8.0 as watchdog
FROM arm64v8/rust:1.45-slim-stretch as builder
WORKDIR /home/rust

ARG ARCH

# Copy all the sources
COPY main ./main

RUN rustup target add aarch64-unknown-linux-gnu
RUN cd main && cargo build --target aarch64-unknown-linux-gnu --release

FROM alpine:3.12 as runner

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates \
    && addgroup -S app && adduser -S -g app app

ENV USER=app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

# Set up watchdog for HTTP mode
ENV fprocess="./main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
