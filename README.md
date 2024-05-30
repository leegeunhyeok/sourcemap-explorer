# sourcemap-explorer

Rust based sourcemap explorer for command-line interface.

## Features

- ⚡️ Light-weight & blazing fast
- 🗺️ Easy to reverse sourcemap mapping
- 🌱 Print original source contents

## Installation

```bash
# TBD
# Go to Releases: https://github.com/leegeunhyeok/sourcemap-explorer/releases
```

## Usage

```bash
sourcemap-explorer ./fixtures/bundle.js.map 1:549 --contents
```

```
Sourcemap explorer

Usage: sourcemap-explorer [OPTIONS] <SOURCEMAP> <POSITION>

Arguments:
  <SOURCEMAP>  Sourcemap file path
  <POSITION>   Position of the source code (eg. 1:549)

Options:
      --contents  Print the original source contents
  -h, --help      Print help
  -V, --version   Print version
```

![preview](image.png)

## Development

```bash
# Run with fixtures
cargo run -- ./fixtures/bundle.js.map 1:549 --contents

# build
cargo build --target xxx --release
```

## License

[BSD 3-Clause](./LICENSE)
