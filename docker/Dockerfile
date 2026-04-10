FROM alpine:3.23

COPY safe_house /usr/bin/
RUN mkdir /app && \
    apk add --no-cache curl && \
    adduser -D safehouse && \
    chmod +x /usr/bin/safe_house && \
    chown safehouse:safehouse /app

EXPOSE 8000
VOLUME /app/data
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -f http://localhost:8000/v1/health || exit

WORKDIR /app
USER safehouse

ENTRYPOINT ["safe_house"]