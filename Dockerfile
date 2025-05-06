FROM rust:1.74-alpine as builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig cmake make gcc git libgcc

WORKDIR /app

# copy all files for 
copy . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates libgcc libstdc++

WORKDIR /app
COPY --from=builder /app/rest_utils/target/release/rest_utils ./api

EXPOSE 8282