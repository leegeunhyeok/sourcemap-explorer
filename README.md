# sourcemap-explorer

Rust based sourcemap explorer for command-line interface.

## Features

- ⚡️ Light-weight & blazing fast
- 🗺️ Easy to reverse sourcemap mapping
- 🌱 Print original source content

## Installation

```bash
# Homebrew
brew tap leegeunhyeok/tap
brew install sourcemap-explorer

# Other platforms: TBD
```

## Usage

```bash
smx ./fixtures/bundle.js.map 1:549 --content
```

```
Sourcemap explorer

Usage: smx [OPTIONS] <SOURCEMAP> <POSITION>

Arguments:
  <SOURCEMAP>  Sourcemap file path
  <POSITION>   Position of the source code (eg. 1:549)

Options:
      --type <TYPE>  Type of runtime [default: default] [possible values: default, hermes]
      --content      Print the original source content
  -h, --help         Print help
  -V, --version      Print version
```

![preview](image.png)

## Development

```bash
# Run with fixtures
cargo run -- ./fixtures/bundle.js.map 1:549 --content

# build
./scripts/build.sh
```

## License

[BSD 3-Clause](./LICENSE)
