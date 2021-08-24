FROM rust:alpine3.13

WORKDIR /usr/src/lcir
COPY . .

RUN apk add --no-cache make build-base
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN rustup component add rls

RUN make deps
