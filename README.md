# `cfg-exclusive`

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

A procedural macro for ensuring that only one of a set of features is enabled at a time.

<!-- vim-markdown-toc GFM -->

* [The Problem](#the-problem)
* [The Solution](#the-solution)
* [License](#license)
    * [Contribution](#contribution)

<!-- vim-markdown-toc -->

Typically, features should be additive. However, there are times when this is not possible or desired.

For such cases, if the number of features is small, or does not change/evolve frequently some verbose `#[cfg]` attributes may suffice.

## The Problem

For example imagine a fictional crate which should only be build with one
of the features `feat1` or `feat2`.

One could create a build script such as the one at `build.rs`.

```rust
fn main() {
    #[cfg(all(feature = "feat1", feature = "feat2"))]
    compile_error!("Only one of the features can be enabled at a time");
}
```

Now imagine, we add a `feat3`.

Our `build.rs` changes to:

```rust
fn main() {
    #[cfg(any(
        all(feature = "feat1", any(feature = "feat2", feature = "feat3")),
        all(feature = "feat2", any(feature = "feat1", feature = "feat3")),
        all(feature = "feat3", any(feature = "feat1", feature = "feat2")),
    ))]
    compile_error!("Only one of the features can be enabled at a time");
}
```

Adding a fourth `feat4` jumps to 12 combinations!

```rust
fn main() {
    #[cfg(any(
        all(feature = "feat1", any(feature = "feat2", feature = "feat3", feature = "feat4")),
        all(feature = "feat2", any(feature = "feat1", feature = "feat3", feature = "feat4")),
        all(feature = "feat3", any(feature = "feat1", feature = "feat2", feature = "feat4")),
        all(feature = "feat4", any(feature = "feat1", feature = "feat2", feature = "feat3")),
    ))]
    compile_error!("Only one of the features can be enabled at a time");
}
```

This gets out of hand quickly.

## The Solution

Starting of with two features, the `cfg-exclusive` procedural macro can be used to simplify the `build.rs` script.

```rust
cfg_exclusive::cfg_exclusive! {
    validate_feats,
    ["feat1", "feat2"],
    "Only one of the features can be enabled at a time"
}

fn main() {
    validate_feats();
}
```

If that changes to three features:

```diff
-    ["feat1", "feat2"],
+    ["feat1", "feat2", "feat3"],
```

Or four:

```diff
-    ["feat1", "feat2", "feat3"],
+    ["feat1", "feat2", "feat3", "feat4"],
```

## License

This crate is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly note otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[crate-image]: https://img.shields.io/crates/v/cfg-exclusive.svg
[crate-link]: https://crates.io/crates/cfg-exclusive
[docs-image]: https://docs.rs/cfg-exclusive/badge.svg
[docs-link]: https://docs.rs/cfg-exclusive
[deps-image]: https://deps.rs/repo/github/kbknapp/cfg-exclusive/status.svg
[deps-link]: https://deps.rs/repo/github/kbknapp/cfg-exclusive

[//]: # (links)
