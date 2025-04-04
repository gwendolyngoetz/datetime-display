FROM --platform=linux/amd64 rust:1.86.0-slim@sha256:9c1ef35ab804dc78361948794f60748e79a7a2e297580604b288590bc52ebdaa as builder
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

