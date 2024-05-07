# specr

This repository contains the specr infrastructure for [MiniRust](https://github.com/minirust/minirust).
specr is a language to write specifications in.
You can think of it as "pseudocode with Rust syntax".

First, there is `specr-transpile`, which converts the specr lang code to actual Rust code, see [here](specr-transpile/README.md) if you are interested in this process.

The generated Rust code makes use of `libspecr` and `gccompat-derive`, which defines a small garbage collector and a few types used in the Minirust spec.
