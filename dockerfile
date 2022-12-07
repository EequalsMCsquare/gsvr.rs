FROM rust:1.65.0-buster as COMPILER
WORKDIR /src
COPY . .
RUN apt-get update
RUN apt-get install protobuf-compiler -y
RUN cargo build --release

FROM rust:1.65.0-slim-buster
WORKDIR /app 
COPY --from=COMPILER /src/target/release/pf /app/pf
COPY --from=COMPILER /src/target/release/gate /app/gate
COPY --from=COMPILER /src/target/release/game /app/game
ENTRYPOINT [ "/app/game" ]