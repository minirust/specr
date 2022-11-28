# specr

specr helps building a `minirust` interpreter.
It does that by transpiling the Pseudorust code to actual Rust code.

The code is WIP, highly unstable.

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

### Automatic Return Wrapping
For functions returning `Option<T>`, `Result<T>` or `Nondet<T>`
it is valid to simply return `T`, which will automatically be wrapped by `Some(_)`, `Ok(_)`, or `Nondet(_)`.

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
    fn foo(&self, x: Some(a)) -> i32 { a }
    fn foo(&self, x: None) -> i32 { 0 }
}
```
Argmatch can also be applied to `self`.

### Forward declarations
Forward declarations in PseudoRust are method implementations missing their code block `{ ... }`.

Example:
```rust
impl Foo {
    fn func(&self);
}
```
specr will clear those forward declarations, and will generally provide an implementation for the method itself.
specr will also implement some methods that have not been forward-declarated, but those should be described in comments.

TODO: explain merge impls or remove it.

TODO: explain module structure.
