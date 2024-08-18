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
FROM nginx:alpine3.20-slim

# Copy over
COPY client/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build-stage /app/dist /var/www/html

# Expose port
EXPOSE 8080

# Run
ENTRYPOINT ["nginx", "-g", "daemon off;"]
