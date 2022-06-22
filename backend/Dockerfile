FROM rust:1.61 as builder
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y musl musl-dev musl-tools

WORKDIR /usr/src/backend
COPY ./backend .
COPY ./db ../db
RUN echo "\n[profile.release-lto]\ninherits = \"release\"\nlto = true" >> ./Cargo.toml
RUN cargo install --target x86_64-unknown-linux-musl --profile release-lto --path .

FROM scratch
EXPOSE 6060
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]
