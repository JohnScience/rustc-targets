# rustc-targets

[![crates.io](https://img.shields.io/crates/v/rustc-targets.svg)][`rustc-targets`]
[![crates.io](https://img.shields.io/crates/d/rustc-targets.svg)][`rustc-targets`]

Library for getting the list of targets supported by rustc.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
rustc-targets = "0.1"
```

after that you can use it like this:

```rust
fn main() {
    let targets = rustc_targets::from_cli().unwrap();
    for (i,target) in targets.iter().unwrap().enumerate() {
        println!("{i}. {target}");
    }
}
```

## Running example

```console
cargo run --example targets
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`rustc-targets`]: https://crates.io/crates/rustc-targets
