# Use the official Rust image as the base image
FROM rust:1.79.0-bookworm

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the working directory
COPY . .

# Build the application
RUN cargo build --release

# Expose the port that the application listens on
EXPOSE 8000

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Set the command to run the application when the container starts
CMD ["cargo", "run", "--release"]