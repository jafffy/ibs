# IBS (iOS Build System)

A command-line tool for iOS project management, written in Rust.

## Requirements

- Rust 1.75 or higher
- Cargo (Rust's package manager)

## Installation

Clone the repository and build from source:

```bash
git clone https://github.com/yourusername/ibs.git
cd ibs
cargo install --path .
```

## Features

- iOS project management from command line
- Project templating and generation
- Build system integration

## Usage

```bash
ibs --help
```

## Development

To build the project locally:

```bash
cargo build
```

For development build:
```bash
cargo run -- [arguments]
```

For release build:
```bash
cargo build --release
```

## License

This project is licensed under the MIT License - see the LICENSE file for details. 