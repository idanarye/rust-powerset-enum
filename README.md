[![Build Status](https://api.travis-ci.org/idanarye/rust-inherent-pub.svg?branch=master)](https://travis-ci.org/idanarye/rust-powerset-enum)
[![Latest Version](https://img.shields.io/crates/v/powerset-enum.svg)](https://crates.io/crates/powerset-enum)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://idanarye.github.io/rust-powerset-enum/)

# Rust Powerset Enum

A poor man's anonymous `enum`, useful mostly for error handling. Turn your
`Error` `enum` into a _Powerset Enum_ to allow taking subsets of the original enums.

See the examples, specifically
[with_powerset_enums.rs](powerset-enum/examples/with_powerset_enums.rs)
vs
[without_powerset_enums.rs](powerset-enum/examples/without_powerset_enums.rs), to understand how this works.

Note: this is a nightly only crate, and to use it you need to enable the
following feature flags:

```rust
#![feature(never_type, exhaustive_patterns, proc_macro_hygiene)]
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
