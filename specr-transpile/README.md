# specr-transpile

specr-transpile converts specr lang code to Rust code.
Run it with `cargo r <config-file>`.

## config file

The config file is a newline-separated list of statements.

Each statement is either

- `input <path>`: where to look for the input .md files.
- `output <path>`: where to generate the output crate.
- `attr <attribute>`: give additional rust crate attributes, like `attr #![feature(never_type)]`

## Current transformations

### Enum Indirection

If you have an enum with infinite size due to type recursion,
you might want to add an indirection as follows:
```rust
enum List<T> {
    Cons {
        val: T,
        #[specr::indirection]
        next: List<T>,
    },
    Nil,
}
```
This will wrap `next` behing a pointer.

Drawbacks:
1. You should not match against references of enums, if you want to use the field with `#[specr::indirection]`.
When matching against `&List<T>` (or `&mut List<T>`) you will see that you still obtain `next` of type `List<T>` and not `&List<T>` (or `&mut List<T>`) as would be correct in Rust.
Hence code like this will not work:
```rust
match &mut l {
    List::Cons { val, next } => {
        *next = Nil;
    },
    _ => {},
}
```
Similarly using `ref` or `ref mut` are not supported for fields behind an `#[specr::indirection]`.

2. Specr does not resolve names first, so there could be naming collisions between variants of different enums and structs.
In the future we intend to warn if such an ambiguity is found.

### Garbage collection and Copy
All types provided by specr like `List`, `Set`, `Map`, `BigInt` are Copy, as they only contain an index into a garbage-collected data structure.
Further, the enum indirection discussed before applies an indirection by using the same garbage collected index.

Hence user-defined types can implement Copy too, they should hence not using non-Copy types from the standard library.
This `#[derive(Copy, Clone)]` is added by specr automatically.

Note that each mutation of a `List`, `Set`, or `Map` currently requires a full clone of that datastructure.
So for example one should use `collect()` over `push()` whenever possible.

TODO: explain how to use the `mark_and_sweep` function.

### Argmatch
methods can match over an argument like so:
```rust
impl Foo {
    #[specr::argmatch(x)]
    fn foo(&self, x: Option<i32>) -> i32;
}

impl Foo {
    fn foo(&self, Some(a): Option<i32>) -> i32 { a }
    fn foo(&self, None: Option<i32>) -> i32 { 0 }
}
```
Argmatch can also be applied to `self`.

### Merge Trait Impls
Whenever a trait implementation is cut into multiple pieces, specr-transpile will merge them back together.

Example:
```rust
trait Foo {
    fn foo1(&self);
    fn foo2(&self);
}

impl Foo for () {
    fn foo1(&self) { ... }
}

impl Foo for () { // invalid in Rust!
    fn foo2(&self) { ... }
}
```

becomes

```rust
trait Foo {
    fn foo1(&self);
    fn foo2(&self);
}

impl Foo for () {
    fn foo1(&self) { ... }
    fn foo2(&self) { ... }
}
```

### Module structure and .md files
specr searches for folders containing markdown files, specr will look in the directory specified by `input` in the config file.
Each folder will result in one Rust module.
This happens by filtering out the rust code of each .md file and concatenating them together.
