FROM rust:buster AS builder

WORKDIR /server

# Accept the build argument
ARG DATABASE_URL

# Make sure to use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /server/target/release/server .

CMD ["./backend"]