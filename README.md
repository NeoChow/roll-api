# Roll API

Roll RPG dice. Used inside of [Power VTT](https://www.poweredvtt.com).

## Routes

### GET /v1/:command

Roll a single type of die. Use [TTML syntax](https://docs.poweredvtt.com/kb/macros#roll) to write
roll functionality.

Examples:

```bash
# Roll a single d20
curl -H 'Content-Type: application/json' 'http://localhost:1337/v1/1d20'

# Reroll anything below a 2
curl -H 'Content-Type: application/json' 'http://localhost:1337/v1/4d8rr<2'

# Set min/min of a die
curl -H 'Content-Type: application/json' 'http://localhost:1337/v1/1d100min2max99'

# Custom sides
curl -H 'Content-Type: application/json' 'http://localhost:1337/v1/1d[0,2,4,6,8,10]'

# Add a comment
curl -H 'Content-Type: application/json' 'http://localhost:1337/v1/1d20[Rolling for gold!]'
```

## Roll Your Own

Looking to run the API locally?

The API can be built using [Rust (nightly)](https://rust-lang.org/) or [Docker](https://docker.com).

Access the API at `http://localhost:1337/`.

### Rust

```bash
# Build
cargo +nightly build --release

# Run
cargo run
```

### Docker

```bash
# Build
docker build -t astral/roll_api:latest .

# Run
docker run -d -p 1337:1337 astral/roll_api:latest
```

# License

[MIT](LICENSE) &copy; 2017-2018 Unicorn Heart Club LLC
