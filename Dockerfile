FROM debian:buster-slim

ENV PATH="${PATH}:/root/.cargo/bin"

RUN apt update
RUN apt install -y build-essential curl libssl-dev pkg-config
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash
RUN apt install -y libssl-dev nodejs

RUN npm install --global yarn

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN cargo install wasm-pack

WORKDIR /workdir
