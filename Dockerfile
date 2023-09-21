FROM alpine:3.18
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
RUN apk add --no-cache git make caddy runit cargo
COPY service /etc/service/
RUN chmod 755 -R /etc/service
ADD . /usr/local/src/solana-sync
WORKDIR /usr/local/src/solana-sync
RUN cargo build --release
RUN mv /usr/local/src/solana-sync/target/release/solana-sync /sbin/solana-sync
CMD ["/sbin/runsvdir", "-P", "/etc/service"]