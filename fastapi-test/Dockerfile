FROM ghcr.io/astral-sh/uv:python3.12-alpine

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PATH="/app/.venv/bin:$PATH"

WORKDIR /app

COPY pyproject.toml README.md uv.lock ./

RUN uv sync --frozen --no-dev --no-install-project

COPY src ./src

RUN uv sync --frozen --no-dev

# Create the data directory for the SQLite volume
RUN mkdir -p /app/data

VOLUME ["/app/data"]

EXPOSE 8000

CMD ["uvicorn", "fastapi_test.main:app", "--host", "0.0.0.0", "--port", "8000"]