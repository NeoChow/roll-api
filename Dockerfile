FROM rustlang/rust:nightly

# add the source
RUN mkdir -p /app/src
ADD Cargo.toml /app
ADD Cargo.lock /app
ADD Rocket.toml /app
ADD src /app/src

# set the source dir
WORKDIR /app

# build source
RUN cargo +nightly build

# start the server
CMD ["cargo", "run"]
