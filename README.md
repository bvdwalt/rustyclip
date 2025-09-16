
# 📋 RustyClip 🦀✂️

[![Crates.io](https://img.shields.io/crates/v/rustyclip.svg)](https://crates.io/crates/rustyclip)
[![Docs.rs](https://docs.rs/rustyclip/badge.svg)](https://docs.rs/rustyclip)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/bvdwalt/rustyclip/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/rustyclip/actions/workflows/ci.yml)
[![CI](https://github.com/bvdwalt/rustyclip/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/rustyclip/actions/workflows/ci.yml)
[![Release](https://github.com/bvdwalt/rustyclip/actions/workflows/release.yml/badge.svg)](https://github.com/bvdwalt/rustyclip/actions/workflows/release.yml)

---

A simple cross-platform **clipboard history manager** written in Rust.  
RustyClip lets you save, search, and reuse snippets you’ve copied to your clipboard.

---

## 🚀 Installation

### Via Cargo (recommended if you have Rust installed)

~~~bash
cargo install rustyclip
~~~

Make sure `$HOME/.cargo/bin` is in your `$PATH` to run `rustyclip` globally.

---

### Prebuilt Binaries

Download the latest release from [GitHub Releases](https://github.com/bvdwalt/rustyclip/releases):

| Platform | Binary |
|----------|--------|
| Linux    | `rustyclip-linux.tar.gz` |
| macOS    | `rustyclip-macos.tar.gz` |
| Windows  | `rustyclip-windows.zip` |

Extract the archive and place the binary somewhere in your `$PATH`.

---

## 🛠 Usage

~~~bash
rustyclip add       # Save the current clipboard text
rustyclip list      # Show saved clipboard history
rustyclip get 0     # Print (and soon restore!) entry at index 0
rustyclip clear     # Clear all saved history
~~~

---

## 📂 Example Workflow

1. Copy some text in your system (e.g., `Ctrl+C` / `Cmd+C`).  
2. Run:  

~~~bash
rustyclip add
~~~

→ Saves it with a timestamp.  

3. See your history:  

~~~bash
rustyclip list
~~~

→ Displays a list of saved entries.  

4. Retrieve an entry:  

~~~bash
rustyclip get 0
~~~

→ Prints it to stdout (future versions will restore it to clipboard automatically).  

---

## 📦 Roadmap

- [x] Save and list clipboard history  
- [x] Clear history  
- [ ] Restore clipboard entries with `get`  
- [ ] Search entries by keyword  
- [ ] Configurable storage path  
- [ ] Hotkey/TUI picker  
- [x] Prebuilt binaries for all platforms  

---

## 🤝 Contributing

Pull requests and feature suggestions are welcome!  
Open an [issue](https://github.com/bvdwalt/rustyclip/issues) to discuss.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
