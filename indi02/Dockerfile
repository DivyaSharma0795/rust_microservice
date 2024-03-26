FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/indi02
COPY . .
RUN cargo install --path .

FROM ubuntu:21.10
COPY --from=builder /usr/local/cargo/bin/indi02 /usr/local/bin/indi02
CMD ["indi02"]