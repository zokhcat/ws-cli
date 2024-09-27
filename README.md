# WS-CLI

A command-line tool for web scraping and automation using Rust.

**Note: Compiles on rust nightly version only**

## Features

- **show_code**: Fetches and prints the raw HTML content of a URL.
- **navigate**: Navigates to a URL using Firefox.
- **capture**: Captures and prints a subsection of the HTML using a CSS selector.
- **click_on**: Clicks on an element using a CSS selector.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)

## Installation

1. Ensure you have **Rust** installed. If not, follow [this link](https://www.rust-lang.org/tools/install) to install it.
2. Clone the repository:
   ```bash
   git clone https://github.com/zokhcat/ws-cli.git
   cd ws-cli
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

**show_code**

Fetches and prints the raw HTML content of a URL.

```sh
cargo run -- show_code https://example.com
```

**navigate**

Navigates to a URL using Firefox.

```sh
cargo run -- navigate https://example.com
```

**capture**

Captures and prints a subsection of the HTML using a CSS selector.

```sh
cargo run -- capture https://example.com h1
```

**click_on**

Clicks on an element using a CSS selector.

```sh
cargo run -- click_on https://example.com button
```

### Future Todos:

- [x] Multi URL handling
- [ ] Configurable Output
- [ ] Verbose Mode
