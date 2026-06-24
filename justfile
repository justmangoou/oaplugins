set shell := ["bash", "-euo", "pipefail", "-c"]

os := `rustc -vV | grep host | cut -d' ' -f2`

plugin-root := "~/.config/opendeck/plugins"

default: build-all

build crate:
    just --justfile crates/{{crate}}/justfile build {{plugin-root}} {{os}}

build-all:
    just build autoclicker
    just build ytmd
