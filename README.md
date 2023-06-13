# minirust-tooling

This repository contains the specr infrastructure for [MiniRust](https://github.com/RalfJung/minirust):

First, there is `specr-transpile`, which converts the specr lang code to actual Rust code, see [here](https://github.com/memoryleak47/minirust-tooling/blob/main/specr-transpile/README.md) if you are interested in this process.

The generated Rust code makes use of `libspecr` and `gccompat-derive`, which defines a small garbage collector and a few types used in the Minirust spec.
