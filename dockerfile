# Inspired by https://kerkour.com/rust-small-docker-image#/from-scratch
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=lynx-scene-generator
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /lynx-scene-generator

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /lynx-scene-generator

# Copy our build
COPY --from=builder /lynx-scene-generator/target/x86_64-unknown-linux-musl/release/lynx-scene-generator ./

# Use an unprivileged user.
USER lynx-scene-generator:lynx-scene-generator

ENTRYPOINT ["/lynx-scene-generator/lynx-scene-generator"]