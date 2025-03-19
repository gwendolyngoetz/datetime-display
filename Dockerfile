FROM --platform=linux/amd64 rust:1.85.1-slim@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim@sha256:bb3dc79fddbca7e8903248ab916bb775c96ec61014b3d02b4f06043b604726dc as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/datetime-display /app/datetime-display
COPY --from=builder /app/public/ /app/public/
COPY --from=builder /app/templates/ /app/templates/

ARG version
ENV VERSION=$version

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT ["/app/datetime-display"]

