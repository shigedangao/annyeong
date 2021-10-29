# build process
# build the dependency independant of the code
FROM rust:1 as builder
ENV APP_PATH=annyeong
RUN USER=root cargo new --bin annyeong
WORKDIR ${APP_PATH}
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Use the code to compile the project w/o having to redownload the dependency
ADD . ./
RUN rm ./target/release/deps/annyeong*
RUN cargo build --release

# actual container which run the app
FROM debian:buster
ARG APP=/usr/src/app

RUN apt-get purge ca-certificates 
RUN apt-get update && apt-get install -y openssl \
    ca-certificates

RUN useradd annyeong

COPY --from=builder /annyeong/target/release/annyeong ${APP}/annyeong

WORKDIR ${APP}

USER annyeong

CMD ["./annyeong"]