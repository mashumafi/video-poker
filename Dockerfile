FROM rust:1.57.0-alpine3.13

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN apk add --update nodejs npm
RUN npm install --global yarn

WORKDIR /workdir
