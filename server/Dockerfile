# context: ../

FROM rust:bookworm AS builder

ARG PROTOC_VERSION=29.3

WORKDIR /app

RUN PROTOC_ZIP="protoc-${PROTOC_VERSION}-linux-x86_64.zip" \
    && curl -fvL -o "/tmp/${PROTOC_ZIP}" \
    "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${PROTOC_ZIP}" \
    && unzip -o "/tmp/${PROTOC_ZIP}" -d /usr/local bin/protoc \
    && unzip -o "/tmp/${PROTOC_ZIP}" -d /usr/local 'include/*' \
    && rm -f "/tmp/${PROTOC_ZIP}"

ENV CARGO_TARGET_DIR=/artifact \
    RUSTUP_HOME=/var/cache/rustup \
    CARGO_HOME=/var/cache/cargo

RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=bind,source=.,target=. \
    rustup show

RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=cache,target=/var/cache/cargo,sharing=locked \
    --mount=type=bind,source=.,target=. \
    cd server && cargo build --release --locked

FROM debian:bookworm-slim AS server-debian-slim

WORKDIR /srv
COPY --from=builder /artifact/release/h24w14 /srv/bin/h24w14

CMD [ "/srv/bin/h24w14" ]
