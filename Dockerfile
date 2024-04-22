FROM rust:latest AS builder
WORKDIR /app


COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 as release
WORKDIR /

COPY --from=builder /app/assets /assets
COPY --from=builder /app/style /style
COPY --from=builder /app/target/release/fileshare /fileshare

EXPOSE 2115

CMD ["/fileshare"]