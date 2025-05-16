FROM --platform=$BUILDPLATFORM ghcr.io/cargo-lambda/cargo-lambda:latest AS build
ARG PACKAGE
ARG TARGETPLATFORM

WORKDIR /build
COPY . .
RUN case "${TARGETPLATFORM}" in \
  "linux/amd64") RUST_ARCH=x86_64-unknown-linux-gnu ;; \
  "linux/arm64") RUST_ARCH=aarch64-unknown-linux-gnu ;; \
  *) exit 1 ;; \
  esac \
  && cargo lambda build --release --target "${RUST_ARCH}" --bin "$PACKAGE"

FROM public.ecr.aws/lambda/provided:al2 AS runtime
ARG PACKAGE

COPY --from=build /build/target/lambda/${PACKAGE}/bootstrap ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD ["app.handler"]
