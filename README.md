# bn [![Crates.io](https://img.shields.io/crates/v/rabe-bn.svg)](https://crates.io/crates/rabe-bn) [![Build status](https://api.travis-ci.org/zcash/bn.svg)](https://travis-ci.org/zcash/bn)

This is a [pairing cryptography](https://en.wikipedia.org/wiki/Pairing-based_cryptography) library written in pure Rust. It makes use of the Barreto-Naehrig (BN) curve construction from [[BCTV2015]](https://eprint.iacr.org/2013/879.pdf) to provide two cyclic groups **G<sub>1</sub>** and **G<sub>2</sub>**, with an efficient bilinear pairing:

*e: G<sub>1</sub> × G<sub>2</sub> → G<sub>T</sub>*

## Security warnings

This library, like other pairing cryptography libraries implementing this construction, is not resistant to side-channel attacks.

## Usage

Add the `bn` crate to your dependencies in `Cargo.toml`...

```toml
[dependencies]
rabe-bn = "0.4.10"
```

...and add an `extern crate` declaration to your crate root:

```rust
extern crate rabe_bn;
```

## API

* `Fr` is an element of F<sub>r</sub>
* `G1` is a point on the BN curve E/Fq : y^2 = x^3 + b
* `G2` is a point on the twisted BN curve E'/Fq2 : y^2 = x^3 + b/xi
* `Gt` is a group element (written multiplicatively) obtained with the `pairing` function over `G1` and `G2`.

## License

Licensed under either of

 * MIT license, ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

Copyright 2016 [Zcash Electric Coin Company](https://z.cash/). The Zcash Company promises to maintain the "bn" crate on crates.io under this MIT/Apache-2.0 dual license.

### Authors

* [Sean Bowe](https://github.com/ebfull)
* [Georg Bramm](https://github.com/georgbramm)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
