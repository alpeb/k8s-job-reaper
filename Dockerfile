# syntax=docker/dockerfile:1.3
FROM rust AS builder

ARG TARGETPLATFORM

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
    cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release && \
    cargo strip && \
    mv /root/target/release/k8s-job-reaper /root


FROM gcr.io/distroless/cc-debian11

COPY --from=builder /root/k8s-job-reaper /

ENTRYPOINT ["./k8s-job-reaper"]
