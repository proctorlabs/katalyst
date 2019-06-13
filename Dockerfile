#This runs the builds...
FROM rust:latest as builder

#We do all this first to generate a good library cache before actually building the final app
ADD Cargo.toml .
ADD katalyst/Cargo.toml katalyst/
ADD katalyst_macros/Cargo.toml katalyst_macros/
RUN mkdir -p katalyst/src katalyst_macros/src && touch katalyst/src/lib.rs && touch katalyst/src/main.rs && touch katalyst_macros/src/lib.rs && \
    (cargo build --release >> /dev/null || true)

#Now we add the actual source and build the app/library itself
ADD katalyst katalyst
ADD katalyst_macros katalyst_macros
RUN rm -rf target/release/deps/*katalyst* && cargo build --release && \
    mkdir -p /pkg/bin /pkg/lib/x86_64-linux-gnu && \
    cp /lib/x86_64-linux-gnu/libgcc_s.so.1 /pkg/lib/x86_64-linux-gnu/libgcc_s.so.1 && \
    cp target/release/katalyst /pkg/bin/katalyst

#Install into target container
FROM gcr.io/distroless/base
COPY --from=builder /pkg/ /
ENTRYPOINT [ "/bin/katalyst" ]
