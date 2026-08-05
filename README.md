# rustfmt-json

Fast JSON formatter, validator, and minifier written in Rust.

## Install

```bash
cargo install --git https://github.com/abirkhan3323-source/rustfmt-json
```

## Usage

```bash
# Pretty-print
rustfmt-json data.json

# Validate only
rustfmt-json --validate data.json

# Minify
rustfmt-json --minify data.json > data.min.json

# Pipe from stdin
cat data.json | rustfmt-json
```

## Options

| Flag | Description |
|------|-------------|
| `-m, --minify` | Output minified JSON |
| `-v, --validate` | Only validate, no output |
| `-h, --help` | Show help |

## Examples
```bash
echo '{"a":1}' | rustfmt-json
rustfmt-json --validate file.json
```
