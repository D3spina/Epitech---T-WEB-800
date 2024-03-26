# Stage de construction
FROM rust:latest as builder
WORKDIR /usr/src/travel

# Copiez tous les membres du workspace ainsi que le fichier de workspace.
COPY ./common ./common
COPY ./services ./services

# Pre-building de travel pour le caching des dépendances
WORKDIR /usr/src/travel/services/travel
RUN cargo build --release

# Stage d'exécution
FROM ubuntu:latest
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/travel/services/travel/target/release/traval /usr/local/bin/travel
EXPOSE 8001

CMD ["travel"]
