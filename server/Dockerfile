# context: ../

FROM node:22 AS frontend-builder

WORKDIR /app

RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    --mount=type=cache,target=/app/node_modules,sharing=locked \
    --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=package-lock.json,target=package-lock.json \
    npm ci

RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    --mount=type=cache,target=/app/node_modules,sharing=locked \
    --mount=type=bind,source=.,target=. \
    npm run build

FROM rust:bookworm AS backend-builder

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
ENV FRONTEND_DIST_DIR /srv/frontend/dist
COPY --from=frontend-builder /app/dist ${FRONTEND_DIST_DIR}
COPY --from=backend-builder /artifact/release/h24w14 /srv/bin/h24w14

CMD [ "/srv/bin/h24w14" ]
