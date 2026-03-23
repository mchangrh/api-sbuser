FROM scratch
ARG TARGETARCH

COPY --chmod=555 docker/${TARGETARCH}/api-sbuser /api-sbuser

ENTRYPOINT ["/api-sbuser"]