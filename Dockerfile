FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create app user
ENV USER=http
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /site

COPY ./ .

RUN mkdir sled_data

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN strip -s /site/target/x86_64-unknown-linux-musl/release/pets

FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /pets

# Copy our build
COPY --from=builder /site/target/x86_64-unknown-linux-musl/release/pets ./pets
COPY --from=builder --chown=http:http /site/sled_data ./sled_data

# Use an unprivileged user.
USER http:http

# Configure rocket
ENV ROCKET_IDENT="Pets Server"
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
EXPOSE 8000

CMD ["/pets/pets"]