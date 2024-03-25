# Stage de construction
FROM rust:latest as builder
WORKDIR /usr/src/sleep

# Copiez tous les membres du workspace ainsi que le fichier de workspace.
COPY ./Cargo.toml ./
COPY ./common ./common
COPY ./services ./services

# Pre-building de eat pour le caching des dépendances
WORKDIR /usr/src/eat/services/sleep
COPY ./common/src ./../../common/src
COPY ./services/sleep/src ./src
RUN cargo build --release

# Stage d'exécution
FROM ubuntu:latest
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/eat/target/release/sleep /usr/local/bin/sleep
EXPOSE 8000

CMD ["eat"]
