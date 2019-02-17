#This runs the builds...
FROM rust:latest as builder

#We do all this first to generate a good library cache before actually building the final app
ADD lib/Cargo.toml lib/Cargo.toml
ADD app/Cargo.toml app/Cargo.toml
ADD Cargo.toml .
RUN mkdir lib/src && mkdir app/src && touch lib/src/lib.rs && touch app/src/main.rs && (cargo build --release >> /dev/null || true)

#Now we add the actual source and build the app/library itself
ADD lib/src lib/src
ADD app/src app/src
ADD config.yml .
RUN rm -rf target/release/deps/libkatalyst* && cargo build --release

#Install into target container
FROM debian:stable-slim
COPY --from=builder ["target/release/katalyst-app", "config.yml", "./"]
ENTRYPOINT [ "./katalyst-app" ]