# TODO: Build rust binary

FROM python:3.12.5-slim AS deploy-stage

WORKDIR /app

# TODO: Copy rust bin over

# Copy, install and cache deps
COPY pyproject.toml .
COPY poetry.lock .
RUN pip install --no-cache-dir -U poetry==1.8.3 && python -m poetry install --no-dev

# Copy source
COPY ./server/src .

# Expose port
EXPOSE 3000

# Run
CMD ["python", "-m", "poetry", "run", "gunicorn", "-b", "0.0.0.0:3000", "-w=4", "main:app"]
