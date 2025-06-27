FROM public.ecr.aws/lambda/provided:al2 AS builder

RUN yum install --assumeyes cargo tar xz
# RUN python3 -m pip install zig

RUN mkdir /opt/bin \
    && curl -o /tmp/zig.tar.xz https://ziglang.org/download/0.14.1/zig-x86_64-linux-0.14.1.tar.xz \
    && tar -C /opt/bin --strip-components=1 -xf /tmp/zig.tar.xz \
    && rm /tmp/zig.tar.xz

RUN cargo install cargo-lambda

WORKDIR /app
COPY . .
RUN cargo lambda build --release --output-format binary --package "lcl-dynamodb-export"

FROM public.ecr.aws/lambda/provided:al2023
ARG PACKAGE
COPY --from=builder /app/target/lambda/${PACKAGE}/bootstrap /var/runtime/bootstrap
CMD ["bootstrap"]
