FROM python:3.12-slim

WORKDIR /app
COPY server.py ./
COPY static ./static

ENV PORT=4173
ENV DATABASE_PATH=/data/navigator.db
RUN mkdir -p /data

VOLUME ["/data"]
EXPOSE 4173
HEALTHCHECK --interval=10s --timeout=3s --retries=5 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://127.0.0.1:4173/readyz')"

CMD ["python", "server.py"]
