FROM public.ecr.aws/lambda/provided:al2
ARG PACKAGE
COPY ${PACKAGE}/bootstrap /var/runtime/bootstrap
CMD ["bootstrap"]
