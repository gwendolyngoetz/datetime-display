FROM --platform=linux/amd64 rust:1.74.0-slim as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/datetime-display /app/datetime-display
COPY --from=builder /app/public/ /app/public/
COPY --from=builder /app/templates/ /app/templates/

ARG version
ENV VERSION=$version

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT ["/app/datetime-display"]

