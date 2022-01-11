FROM rust:1.57.0-alpine3.13

apk add --update nodejs npm
npm install --global yarn

