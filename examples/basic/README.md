# Example

This fictional example demonstrates a crate which should only be build with one
of the features `feat1`, `feat2`, or `feat3`. Presumably they all provide the same
functionality and cannot overlap for some reason.

One would create a build script such as the one at `build.rs`.

To test that it does indeed fail to comiple with two or more of the conflicting
features, all of the following should fail to compile:

```sh
cargo build --features feat1,feat2
cargo build --features feat2,feat3
cargo build --features feat1,feat3
cargo build --features feat1,feat2,feat3
```

Building without conflicting features should work; all of these should compile:

```sh
cargo build --features feat1
cargo build --features feat2
cargo build --features feat3
```

## Details

The `build.rs` script is as follows:

```rust
use cfg_exclusive::cfg_exclusive;

cfg_exclusive! {
    validate_feats,
    ["feat1", "feat2", "feat3"],
    "Only one of the features can be enabled at a time"
}

fn main() { validate_feats(); }
```

`cfg_exclusive` takes three arguments:

* A function name that you can call which will generate a `compile_error!` if the
  features are not exclusive.
* A list of features that should be exclusive.
* A message to display if the features are not exclusive.

The generated code is roughly equivalent to:

```rust
use cfg_exclusive::cfg_exclusive;

fn validate_feats() {
    #[cfg(any(
        all(feature = "feat1", any(feature = "feat2", feature = "feat3")),
        all(feature = "feat2", any(feature = "feat1", feature = "feat3")),
        all(feature = "feat3", any(feature = "feat1", feature = "feat2")),
    ))]
    compile_error!("Only one of the features can be enabled at a time");
}

fn main() { validate_feats(); }
```
