FROM ghcr.io/cargo-lambda/cargo-lambda AS builder

# RUN yum install --assumeyes cargo
# RUN cargo install cargo-lambda
WORKDIR /app
COPY . .
RUN cargo lambda build --release --output-format binary --package "lcl-dynamodb-export"

FROM public.ecr.aws/lambda/provided:al2
ARG PACKAGE
COPY --from=builder /app/target/lambda/${PACKAGE}/bootstrap /var/runtime/bootstrap
CMD ["bootstrap"]
