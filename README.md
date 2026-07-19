<div align="center">

# nix-activate

[![version](https://badgen.net/github/tag/sudosubin/nix-activate?label=version)](https://github.com/sudosubin/nix-activate/tags)
[![license](https://badgen.net/github/license/sudosubin/nix-activate?color=green)](LICENSE)

A simple script to enable nix flake direnv from current directory or external flake repository.

</div>

## Quick Start

```sh
export NIX_ACTIVATE_ROOT=/path/to/flakes
nix-activate
direnv allow
```

## Installation

```sh
cargo install --git https://github.com/sudosubin/nix-activate
```

Or install with Homebrew:

```sh
brew install sudosubin/tap/nix-activate
```

## Configuration

Set `NIX_ACTIVATE_ROOT` to the directory that stores external flake repositories.

## How It Works

1. Checks the current directory's flake.
   - Example: `use flake .`
2. Checks the matching git repository flake.
   - Example: `use flake /path/to/flakes/github.com/sudosubin/nix-activate`
   - Git subdirectories are supported: `use flake /path/to/flakes/github.com/sudosubin/nix-activate/v2`
3. Checks the matching local-only flake.
   - Example: `use flake /path/to/flakes/local/sudosubin/nix-activate`

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --all-features
```

## License

MIT, see [LICENSE](./LICENSE).
