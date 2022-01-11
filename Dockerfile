FROM debian:buster-slim

ENV PATH="${PATH}:${HOME}/.cargo"

RUN apt update
RUN apt install -y curl
RUN curl -fsSL https://deb.nodesource.com/setup_17.x | bash -
RUN apt install -y curl nodejs npm

RUN npm install --global yarn

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN $HOME/.cargo/bin/rustup toolchain install nightly
RUN $HOME/.cargo/bin/rustup default nightly
RUN $HOME/.cargo/bin/cargo install wasm-pack

WORKDIR /workdir
