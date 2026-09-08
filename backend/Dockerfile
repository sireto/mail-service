# Pinned rather than `rust:latest`: an unpinned builder makes the image non-reproducible
# and can break the build without a commit. Must stay >= the MSRV of Cargo.lock.
FROM rust:1.95-bookworm AS builder

WORKDIR /app

# Dependencies first so a source-only change does not refetch the registry.
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

COPY src ./src
COPY migrations ./migrations

# --no-default-features drops the `mocks` feature, so the mockall-generated test scaffolding
# is not compiled into the production binary.
RUN cargo build --release --locked --no-default-features

FROM debian:bookworm-slim

# libpq5 is the runtime library; the previous image installed libpq-dev, which is the
# development package and pulls in headers the running service has no use for.
RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq5 ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Run as an unprivileged user. The previous image ran everything as root.
RUN useradd --system --create-home --shell /usr/sbin/nologin appuser

WORKDIR /app
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

USER appuser

EXPOSE 8000

# The health endpoint now checks the database, so it is meaningful as a container probe.
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl -fsS http://127.0.0.1:8000/api/health || exit 1

CMD ["backend"]
