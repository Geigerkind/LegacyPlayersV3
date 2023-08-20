FROM rust:1.67

RUN rustup install nightly-2023-02-07
RUN rustup default nightly-2023-02-07

RUN mkdir /Backend
COPY ./src /Backend/src
COPY ./sub_crates /Backend/sub_crates
COPY ./Cargo.toml /Backend/Cargo.toml

WORKDIR /Backend
RUN cargo build --release

ENTRYPOINT ["/bin/bash", "-c", "sleep 5s && /Backend/target/release/backend"]