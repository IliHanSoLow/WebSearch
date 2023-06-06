# WebSearch

Terminal Websearch written in Rust inspired by OhMyZsh's web-search

---

## Installation
Either go to the release page and follow the instructions, or build from source:
### From Release
Get a release from the release page put the file wherever you want and add the folder it is in to your Path.
### From Source
```
git clone https://github.com/IliHanSoLow/WebSearch
cd WebSearch
cargo build --release
mkdir -p ~/.local/bin
cp target/release/websearch ~/.local/bin
```
add `~/.local/bin` to your path 

## Usage:
1. Get a release from the release page and add the file to your Path
```
websearch {yourSearchmachine} {YourQuery}
```
### Example:
```
$ websearch google test
```
