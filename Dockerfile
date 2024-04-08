# Stage de construction
FROM rust:latest@sha256:84d4e88a86481073bf876770768632d8c7783fc58f14fbd67b387f75f889db23 as builder

# Copiez tous les membres du workspace ainsi que le fichier de workspace.
# Pre-building de eat pour le caching des dépendances
WORKDIR /usr/src/drink
COPY ./Cargo.toml ./Cargo.toml
COPY ./common ./common
COPY ./services ./services
COPY .env .env

# Création du build release
RUN cargo build --release


# Stage d'exécution
FROM debian:bookworm@sha256:c2cedd7f80a4dd0f9f80d3699bd433ccf3de33ab63bfa2d4c4ba870c998222d6
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/drink/target/release/drink /usr/local/bin/drink
EXPOSE 8003

CMD ["drink"]
