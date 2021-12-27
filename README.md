# RΞBLCK
A revival and reimplemention of [BLCK](https://github.com/FlowVix/BLCK) in Rust.

## What version of [SPWN](https://github.com/Spu7Nix/SPWN-language) is used?
The latest "stable" version. As of writing this is v0.7.

## Warning
**RΞBLCK is in very early development and currently is not functional.**

## Dependencies
- Cargo
- [sdl2](https://libsdl.org/download)
- [sdl2\_gfx](https://sourceforge.net/projects/sdl2gfx/)

SDL2 install on Linux (Arch based)
```sh
$ pacman -S sdl2 sdl2_gfx
```

## Building
```sh
$ git clone --recursive https://github.com/Deltara3/REBLCK;cd REBLCK
$ cargo build --release
```
The built binary will be at `target/release/reblck`.
