# Stage de construction
FROM rust:latest as builder

# Copiez tous les membres du workspace ainsi que le fichier de workspace.
# Pre-building de drink pour le caching des dépendances
WORKDIR /usr/src/drink
COPY ./Cargo.toml ./Cargo.toml
COPY ./common ./common
COPY ./services ./services
COPY .env .env

# Création du build release
RUN cargo build --release

# Stage d'exécution
FROM ubuntu:latest
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/drink/target/release/drink /usr/local/bin/drink
EXPOSE 8003

CMD ["drink"]
