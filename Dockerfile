FROM --platform=linux/amd64 rust:1.86.0-slim@sha256:3f391b0678a6e0c88fd26f13e399c9c515ac47354e3cadfee7daee3b21651a4f as builder
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

