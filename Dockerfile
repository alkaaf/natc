FROM rust:latest
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=0 /target/release/natc .
RUN chmod +x natc
CMD ["./natc"]