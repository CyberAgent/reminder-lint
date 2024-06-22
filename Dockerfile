FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:aarch64-musl-${TARGETARCH} AS builder

ARG TARGETARCH

WORKDIR /build

RUN if [ $TARGETARCH = "amd64" ]; then \
      echo "x86_64" > /arch; \
    elif [ $TARGETARCH = "arm64" ]; then \
      echo "aarch64" > /arch; \
    else \
      echo "$TARGETARCH is not supported"; \
      exit 1; \
    fi

COPY . .

RUN rustup target add $(cat /arch)-unknown-linux-musl && \
    cargo build --package reminder-lint --locked --release --target $(cat /arch)-unknown-linux-musl

RUN cp target/$(cat /arch)-unknown-linux-musl/release/reminder-lint /build/reminder-lint


FROM --platform=$TARGETPLATFORM debian:bullseye-slim

COPY --from=builder /build/reminder-lint /usr/local/bin/reminder-lint/cli

ENTRYPOINT [ "/usr/local/bin/reminder-lint/cli" ]
