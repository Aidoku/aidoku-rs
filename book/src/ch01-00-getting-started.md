# Getting Started

At a high level, Aidoku sources use network requests to read data from websites and parse it into
content that the Aidoku application can read and display to users. A source can be written for any
website that provides content that can be grouped into "series" (or, manga) and "chapters" that have
"pages" that can be read (plain text content, or images).

Aidoku sources are [WebAssembly](https://webassembly.org/) programs, and the
[aidoku-rs](https://github.com/Aidoku/aidoku-rs) library and
[aidoku cli](https://github.com/Aidoku/aidoku-rs/tree/main/crates/cli) tool enable you to write Rust
programs that can be compiled into source packages that the Aidoku app runs.
