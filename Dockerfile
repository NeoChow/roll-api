FROM rustlang/rust:nightly

# add the source
RUN mkdir -p /app/src
ADD Cargo.toml /app
ADD Rocket.toml /app
ADD src /app/src

# set the source dir
WORKDIR /app

# build source
RUN cargo +nightly build --release

# expose the rust api server
EXPOSE 1337

# start the server
CMD ["cargo", "run", "--release"]
