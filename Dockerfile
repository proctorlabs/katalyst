#This runs the builds...
FROM rust:latest as builder

#We do all this first to generate a good library cache before actually building the final app
ADD Cargo.toml .
RUN mkdir -p src/app && touch src/lib.rs && touch src/app/app.rs && (cargo build --release >> /dev/null || true)

#Now we add the actual source and build the app/library itself
ADD src src
RUN rm -rf target/release/deps/libkatalyst* && cargo build --release

#Install into target container
FROM debian:stable-slim
COPY --from=builder ["target/release/katalyst", "./"]
ENTRYPOINT [ "./katalyst" ]