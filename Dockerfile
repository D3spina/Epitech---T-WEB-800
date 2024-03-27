# Stage de construction
FROM rust:latest as builder

# Copiez tous les membres du workspace ainsi que le fichier de workspace.
# Pre-building de sleep pour le caching des dépendances
WORKDIR /usr/src/sleep
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
COPY --from=builder /usr/src/sleep/target/release/sleep /usr/local/bin/sleep
EXPOSE 8004

CMD ["sleep"]
