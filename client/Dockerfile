## Stage 1: Build
FROM node:22.6.0-alpine3.20 AS build-stage

WORKDIR /app

# Copy, install and cache deps
COPY client/package*.json ./
RUN npm install -dd

# Copy and build source
COPY client/ .
RUN npm run build


## Stage 2: Run
FROM busybox:1.36.1 AS deploy-stage

# Create user
RUN adduser -D static
USER static

WORKDIR /home/static
COPY --from=build-stage /app/dist/ .

# Expose port
EXPOSE 8080

# Run
CMD ["busybox", "httpd", "-f", "-v", "-p", "8080"]
