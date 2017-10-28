FROM ubuntu:artful

RUN apt-get update \
    && apt-get install -y qt5-default build-essential curl cmake gcc python3 python3-pip libssl-dev ca-certificates pkg-config \
    && apt-get autoremove \
    && apt-get autoclean

RUN pip3 install ghp-import

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.21.0
RUN cargo install mdbook

RUN pip3 install awscli 

WORKDIR /code