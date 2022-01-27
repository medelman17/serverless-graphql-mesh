FROM --platform=$BUILDPLATFORM rust:latest as builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN echo "I am running on $BUILDPLATFORM, building for $TARGETPLATFORM" > /log

ARG target=x86_64-unknown-linux-musl
ARG binary=bootstrap
ARG user=graphql

RUN rustup target add $target

RUN apt-get update && apt-get install -y \
    libv8-dev \
    musl-dev \
    libssl-dev \
    build-essential \
    libglib2.0-dev

RUN update-ca-certificates

ENV USER=$user
ENV UID=10001
ENV TARGET=$target
ENV BINARY=$binary


RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

# RUN cargo install sccache

RUN wget https://ziglang.org/download/0.9.0/zig-linux-x86_64-0.9.0.tar.xz
RUN tar xvf zig-linux-x86_64-0.9.0.tar.xz


# ENV RUSTC_WRAPPER="$HOME/.cargo/sccache"

ENV CC="/app/zcc"
ENV CXX="/app/zxx"

COPY ./ .


RUN cargo build --target $target --release

FROM scratch

ARG target=x86_64-unknown-linux-musl
ARG binary=bootstrap

#COPY --from=builder /etc/passwd /etc/passwd
#COPY --from=builder /etc/group /etc/group

WORKDIR .

COPY --from=builder "/app/target/$target/release/$binary" .

USER $USER:$USER
