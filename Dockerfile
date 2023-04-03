FROM rust:latest

WORKDIR /usr/src/website

COPY *.toml ./
# not sure if the lock file needs to come with, but it would prevent the inability to
# build if a crate gets yanked but the project hasn't been updated 
COPY Cargo.lock ./
COPY src/ ./src
# To put `www/` in the docker image `scp` (ssh copy) it to the server then `docker cp` it into the image

RUN cargo install --path .

# Port as defined in Rocket.toml
EXPOSE 8000
# Name as defined in Cargo.toml
CMD ["player_logger"]