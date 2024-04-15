FROM rust:1.77.0

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for linking configuration
RUN apt update && apt install lld clang -y
#Copy all files form our working directory to our Docker image
COPY . .
# Lets build our binary!
# We'll use the release profile to make it faaaaast
ENV SQLX_OFFLINE true
RUN cargo build --release
# When `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]
