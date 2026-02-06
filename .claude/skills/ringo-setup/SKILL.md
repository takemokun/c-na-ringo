---
name: ringo-setup
description: Build and install the ringo-srs CLI tool. Run this once after cloning the repo.
---

# Ringo Setup

Build the `ringo-srs` Rust CLI and install it to `./bin/ringo-srs`.

## Workflow

### 1. Check Prerequisites

```bash
cargo --version
```

If `cargo` is not found, stop and tell the user:
> "Rust toolchain is required. Install it with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`"

### 2. Run Setup

```bash
make setup
```

This runs test, build, install, and verification in one command.

### 3. Report Result

Show the `make setup` output to the user. On failure, report which step failed.

## Notes

- This is a setup/build skill, not a language learning skill (no scope lockdown)
- Actual steps are defined in `Makefile` (`make setup` = test + install + verify)
- `bin/`, `.crates.toml`, `.crates2.json` are gitignored
