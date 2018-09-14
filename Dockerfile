FROM ubuntu:trusty

RUN mkdir /code
WORKDIR /code
RUN apt-get update
RUN apt-get install -y curl
RUN apt-get install -y gcc-arm-linux-gnueabihf

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN touch ~/.cargo/config
RUN echo '[target.armv7-unknown-linux-gnueabihf]' >> ~/.cargo/config
RUN echo 'linker = "arm-linux-gnueabihf-gcc"' >> ~/.cargo/config
