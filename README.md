# Game of Life - Rust

![Action Status](https://github.com/boechat107/game-of-life-rust/workflows/Rust/badge.svg)

A simple implementation of Conway's Game of Life in Rust.

[![asciicast](https://asciinema.org/a/514433.svg)](https://asciinema.org/a/514433)

## Installation

Compile and run with [Cargo](https://doc.rust-lang.org/cargo/).

## Usage

The program runs in a infinite loop.

``` bash
cargo run < ./seeds/PulsarSeed.txt
```

Stop it with `Ctrl-C`.

## References

* [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway's_Game_of_Life)
* amhndu's [implementation in Haskell](https://github.com/amhndu/life.hs)
   * Elegant solution.
   * Contains the original seed files of this repository.
