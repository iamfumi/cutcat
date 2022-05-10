FROM rust:1.60 AS builder
WORKDIR /home/cutcat
COPY . .
RUN adduser --home /home/cutcat --shell /bin/false cutcat ¥
  && cargo build --release ¥
  && cp target/release/cutcat /usr/local/bin/cutcat
USER cutcat
ENTRYPOINT [ "/usr/local/bin/cutcat" ]
