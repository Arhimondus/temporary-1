FROM rust:1.74.0 AS deps

WORKDIR /var
RUN USER=root cargo new app
WORKDIR /var/app
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN cargo build --release && rm -rf src && rm -rf target/release/$(grep -Po '(?<=^name = ")[^"]*(?=".*)' ./Cargo.toml)

FROM deps AS build

WORKDIR /var/app
COPY src ./src/
RUN touch src/main.rs && cargo build --release
RUN mv ./target/release/$(grep -Po '(?<=^name = ")[^"]*(?=".*)' ./Cargo.toml) ./binfile
RUN ln -s /usr/lib/libssl.so.1 /usr/lib/libssl.so.3
RUN ln -s /usr/lib/libssl.so.3 /usr/lib/libssl.so.3.3
RUN ln -s /usr/lib/libssl.so.3 /usr/lib/libssl.so
RUN ln -s /usr/lib/libnsl.so.1 /usr/lib/libnsl.so.1.1
RUN chmod +x ./binfile

CMD ./binfile