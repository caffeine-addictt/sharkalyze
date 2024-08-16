## Stage 1: Build Rust
FROM rust:1.80.1-slim AS builder-stage

WORKDIR /app

# Set build target and cache deps
COPY Cargo.toml Cargo.lock ./
COPY parser/Cargo.toml ./parser/
RUN \
  mkdir parser/src \
  && echo "fn main() {}" > parser/src/main.rs
RUN cargo build --release

# Copy source and build
COPY parser/src ./parser/src
RUN \
  cargo build --release \
  && strip target/release/parser


## Stage 2: Deploy
FROM python:3.12.5-slim AS deploy-stage

WORKDIR /app

# Copy rust bin over
COPY --from=builder-stage /app/target/release/parser .

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
