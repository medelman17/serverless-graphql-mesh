ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG SOURCE_DIR="."
ARG BINARY_NAME="bootstrap"
ARG USER=lambda
ARG TARGET=aarch64-unknown-linux-gnu

FROM --platform=$BUILDPLATFORM rust:latest as builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    libssl-dev

ARG USER
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

ARG TARGET

RUN rustup target add $TARGET

ARG SOURCE_DIR
ARG BINARY_NAME

WORKDIR /app

COPY ${SOURCE_PATH} .

RUN cargo build --target $TARGET --release


FROM --platform=$BUILDPLATFORM public.ecr.aws/lambda/provided:latest
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

ARG BINARY_NAME
ARG TARGET

WORKDIR ${LAMBDA_RUNTIME_DIR}

COPY --from=builder "/app/target/$TARGET/release/$BINARY_NAME" .
COPY --from=builder "/app/target/$TARGET/release/extension" /opt/extensions


CMD ["bootstrap"]
