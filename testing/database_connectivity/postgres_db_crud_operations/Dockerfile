# Use an official Rust runtime as a parent image
FROM rust:latest

# # Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the source code into the container
COPY . .

# Build the application with dependencies
RUN cargo build --release

# # Expose any necessary ports (if your app listens on a port)
# EXPOSE 8080

# # Define the command to run your application when the container starts
# CMD ["cargo run"]

CMD cargo run
