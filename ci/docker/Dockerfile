# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM alpine as tiny-project

# Use BuildKit to help translate architecture names
ARG TARGETPLATFORM

COPY artifacts/ /artifacts
WORKDIR /barnacle-bin

RUN case "${TARGETPLATFORM}" in \
         "linux/amd64")  TARGET_DIR=x86_64-unknown-linux-musl  ;; \
         "linux/386")    TARGET_DIR=i686-unknown-linux-musl   ;; \
         "linux/arm64")  TARGET_DIR=aarch64-unknown-linux-musl  ;; \
         "linux/arm/v7") TARGET_DIR=armv7-unknown-linux-musleabihf  ;; \
         *) exit 1 ;; \
    esac; \
    cp /artifacts/${TARGET_DIR}/barnacle .

FROM scratch
COPY --from=tiny-project --chmod=0755 /barnacle-bin/barnacle /barnacle
ENTRYPOINT ["/barnacle"]
