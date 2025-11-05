FROM docker.io/library/rust:slim-trixie AS backend-builder
WORKDIR /builder
RUN apt update && apt install build-essential curl wget file libssl-dev pkg-config -y
COPY . .
RUN cargo build --all -r

FROM docker.io/library/debian:trixie-slim AS backend
WORKDIR /app
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=backend-builder /builder/target/release/backend /app/
CMD [ "./backend" ]

