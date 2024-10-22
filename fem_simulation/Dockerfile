# Use a base image with Node.js for serving the web app
FROM node:14-alpine

# Install wasm-pack
RUN apk add --no-cache pkgconfig openssl-dev build-base \
    && cargo install wasm-pack

WORKDIR /usr/src/app

# Copy the Rust source code
COPY . .

# Build the WebAssembly module
RUN wasm-pack build --target web

# Install a simple HTTP server to serve files
RUN npm install -g http-server

# Expose port 8080
EXPOSE 8080

# Start the HTTP server
CMD ["http-server", ".", "-p", "8080"]
