FROM rust:1.57.0-alpine3.13

RUN apk add --update nodejs npm
RUN npm install --global yarn

WORKDIR /workdir
