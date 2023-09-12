FROM rust:latest as build

# RUN mkdir -p ./server/
# WORKDIR ./server/

COPY ./ ./

RUN cargo build --release

expose 3000

CMD ["./target/release/server"]
