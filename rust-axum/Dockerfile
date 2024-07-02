FROM rust:1.79 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM rust:1.79-slim AS runtime

RUN apt-get update && apt-get install -y postgresql-client

COPY --from=builder /usr/src/app/target/release/pass-in /usr/local/bin/

EXPOSE 3333

CMD ["pass-in"]
