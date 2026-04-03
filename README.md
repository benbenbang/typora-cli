# typora-cli

A minimal cross-platform CLI wrapper that opens files in [Typora](https://typora.io). It creates the file first if it doesn't exist, so you can go straight from the terminal to writing.

> this is a small toy for practicing Rust 🦀 nothing fancy.

## Usage

```sh
typora-cli                  # open current directory in Typora
typora-cli .                # same as above
typora-cli notes.md         # open (or create) notes.md in the current directory
typora-cli notes            # open (or create) a file called "notes" — no extension is added
typora-cli docs/readme.md   # create any missing parent directories, then open
typora-cli ../shared.md     # relative paths work too
typora-cli /abs/path/to/file.md  # absolute paths work too
```

The file is created with exactly the name you provide — no extension is appended automatically.

## Installation

### Prerequisites

- [Typora](https://typora.io) must be installed.
- On **Linux** and **Windows**, `typora` must be on your `PATH`.
- On **macOS**, `open -a typora` is used, so PATH is not required.

### Build from source

```sh
cargo install --path .
```

Or just build and copy the binary manually:

```sh
cargo build --release
cp target/release/typora-cli /usr/local/bin/
```

## Platform support

| Platform | How Typora is launched         |
|----------|-------------------------------|
| macOS    | `open -a typora <path>`       |
| Linux    | `typora <path>` (PATH-based)  |
| Windows  | `typora <path>` (PATH-based)  |

## How it works

1. No argument or `"."` → opens the current directory in Typora.
2. Any other argument → the path is used as-is:
   - Missing parent directories are created automatically.
   - If the file does not exist it is created (equivalent to `touch`).
   - The file is then opened in Typora using its absolute path.
