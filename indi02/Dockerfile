FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/indi02
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/indi02 /usr/local/bin/indi02
CMD ["indi02"]