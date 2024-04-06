FROM rust:1.77-alpine3.19 as builder
RUN apk update && \
  apk upgrade && \
  apk add musl-dev=1.2.4_git20230717-r4 upx=4.2.1-r0 --no-cache
WORKDIR /var/tmp
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl && \
  upx --best --lzma target/x86_64-unknown-linux-musl/release/secure-conductor

FROM alpine:3.19 AS runtime
RUN apk update && \
  apk upgrade --update-cache --available && \
  rm -rf /var/cache/apk/*
WORKDIR /secure-conductor
EXPOSE 8080
COPY --from=builder /var/tmp/target/x86_64-unknown-linux-musl/release/secure-conductor usr/local/bin/secure-conductor
ENTRYPOINT ["./usr/local/bin/secure-conductor"]
