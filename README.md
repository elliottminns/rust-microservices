# Rust Web Frameworks

This repo is part of a blog post which looks into different Rust frameworks.

https://starstorm.dev/post/rust-webframeworks/

## Usage

To run a webframwork, use the following command

```bash
$ cargo run --bin <framework>
```

There are a few frameworks to make use of:

```
actix
poem
rocket
tide
warp
```

i.e.

```
$ cargo run --bin tide
```

This will spin up the web framework service, which is then able to be tested
using the validator.

To validate the framework, in a new window type:

```
$ cargo run --bin validator
```
