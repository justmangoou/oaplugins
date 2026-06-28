set shell := ["bash", "-euo", "pipefail", "-c"]

os := `rustc -vV | grep host | cut -d' ' -f2`

plugin-root := "~/.config/opendeck/plugins"

default: build-all

check:
    deno task --recursive check

build-all: check
    just build macro
    just build ytmd

build crate:
    just --justfile crates/{{crate}}/justfile build {{plugin-root}} {{os}}
