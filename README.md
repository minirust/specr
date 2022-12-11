# minirust-tooling

This repository contains a bunch of crates used to work with minirust.
Note that these are still WIP.

First, there is `specr-transpile`, which converts the PseudoRust code to actual Rust code, see [here](https://github.com/memoryleak47/minirust-tooling/blob/main/specr-transpile/README.md) if you are interested in this process.

The `minirust` code has to lie in `./minirust` within this repository, whereas the generated Rust-code will be written to `./gen-minirust` by `specr-transpile`.

The generated Rust code makes use of `libspecr`, which defines a small garbage collector and a few types used in the Minirust spec.

Last but not least, there is `minimize` which allows to convert Rust code to MiniRust code, and then execute it.
