# Power VTT Roll API

Roll RPG dice. Used inside of [Power VTT](https://www.poweredvtt.com).

## Routes

### GET /v1/:command

Roll a single type of die. Use [TTML syntax](https://wiki.poweredvtt.com/macros#roll) to extend
roll functionality.

Examples:

```bash
# Roll a single d20
curl -H "Content-Type: application/json" https://roll.poweredvtt.com/1d20

# Reroll anything below a 2
curl -H "Content-Type: application/json" https://roll.poweredvtt.com/4d8rr2

# Set min/min of a die
curl -H "Content-Type: application/json" https://roll.poweredvtt.com/1d100min2max99
```

## Roll Your Own

Looking to run the roll API locally? You will need [Rust (nightly)](https://rust-lang.org/).

```bash
cargo +nightly build
```

You can access the server at `http://localhost:1337/`.

# License

[MIT](LICENSE) &copy; 2017 Unicorn Heart Club LLC
