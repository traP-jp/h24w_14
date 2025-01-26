# context: ../

FROM node:22 AS frontend-builder

WORKDIR /app/client

COPY client/package.json client/package-lock.json /app/client/
RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    --mount=type=cache,target=/app/node_modules,sharing=locked \
    npm i

COPY client /app/client
RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    --mount=type=cache,target=/app/node_modules,sharing=locked \
    npm run build

FROM rust:bookworm AS backend-builder

ARG PROTOC_VERSION=29.3

WORKDIR /app/server

RUN PROTOC_ZIP="protoc-${PROTOC_VERSION}-linux-x86_64.zip" \
    && curl -fvL -o "/tmp/${PROTOC_ZIP}" \
    "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${PROTOC_ZIP}" \
    && unzip -o "/tmp/${PROTOC_ZIP}" -d /usr/local bin/protoc \
    && unzip -o "/tmp/${PROTOC_ZIP}" -d /usr/local 'include/*' \
    && rm -f "/tmp/${PROTOC_ZIP}"

ENV CARGO_TARGET_DIR=/artifact
ENV RUSTUP_HOME=/var/cache/rustup
ENV CARGO_HOME=/var/cache/cargo

RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=bind,source=./server,target=/app/server \
    --mount=type=bind,source=./proto,target=/app/proto \
    rustup show

RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=cache,target=/var/cache/cargo,sharing=locked \
    --mount=type=bind,source=./server,target=/app/server \
    --mount=type=bind,source=./proto,target=/app/proto \
    cargo build --release --locked

FROM debian:bookworm-slim AS server-debian-slim

WORKDIR /srv
ENV FRONTEND_DIST_DIR=/srv/frontend/dist
COPY --from=frontend-builder /app/client/dist ${FRONTEND_DIST_DIR}
COPY --from=backend-builder /artifact/release/h24w14 /srv/bin/h24w14

CMD [ "/srv/bin/h24w14" ]
