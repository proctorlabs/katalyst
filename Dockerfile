#This runs the builds...
FROM rust as builder

ADD lib/src lib/src
ADD app/src app/src
ADD lib/Cargo.toml lib/Cargo.toml
ADD app/Cargo.toml app/Cargo.toml

RUN cd app && cargo build --release
ADD app/config.yml .
RUN mv app/target/release/katalyst-app .


#Install into target container
FROM debian:stable-slim
COPY --from=builder ["katalyst-app", "config.yml", "./"]
ENTRYPOINT [ "./katalyst-app" ]