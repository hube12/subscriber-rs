# subscriber-rs

[![Crates.io](https://img.shields.io/crates/v/subscriber-rs.svg)](https://crates.io/crates/subscriber-rs)
[![Docs.rs](https://docs.rs/subscriber-rs/badge.svg)](https://docs.rs/subscriber-rs)
[![CI](https://github.com/hube12/subscriber-rs/workflows/CI/badge.svg)](https://github.com/hube12/subscriber-rs/actions)
[![Coverage Status](https://coveralls.io/repos/github/hube12/subscriber-rs/badge.svg?branch=main)](https://coveralls.io/github/hube12/subscriber-rs?branch=main)

A simple subscriber framework see `examples/simple.rs` for How-To.

This crates solves the problems of how to register a callback (maybe FFI one) and send in an event loop different events.


Note: the no_std part is not working atm and we depends of tokio for the executor, we will switch to agnostik later on.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install subscriber-rs`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
