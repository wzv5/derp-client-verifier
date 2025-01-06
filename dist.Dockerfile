FROM gcr.io/distroless/cc:latest
WORKDIR /app
COPY derp-client-verifier .
ENTRYPOINT ["/app/derp-client-verifier"]
