# WebSearch

Terminal Websearch written in Rust inspired by OhMyZsh's web-search

---

## Installation
Either go to the release page and follow the instructions or build from source:
### Linux
```
git clone https://github.com/IliHanSoLow/WebSearch
cd WebSearch
cargo build --release
mkdir -p ~/.local/bin
cp target/release/websearch ~/.local/bin
```
add `~/.local/bin` to your path 

## Usage:
1. Get a release from the release Page and add the file to your Path
2. Use: <br>
```
websearch {yourSearchmachine} {YourQuery}
```
### Example:
```
$ websearch github IliHanSoLow
```
