FROM rust:latest as builder
WORKDIR /app
copy huereader .
RUN cargo build

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/huereader
CMD ["/app/huereader"]
