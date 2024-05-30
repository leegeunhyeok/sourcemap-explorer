# sourcemap-explorer

Rust based sourcemap explorer for command-line interface.

## Features

- ⭐️ Light-weight & blazing fast
- 🗺️ Easy to reverse sourcemap mapping

## Installation

```bash
# TBD
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

## License

[BSD 3-Clause](./LICENSE)
