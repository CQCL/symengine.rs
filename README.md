# SymEngine Rust Wrappers

Unofficial Rust wrappers to the C++ library [SymEngine](https://github.com/symengine/symengine), a fast C++ symbolic manipulation library.

## Usage

### Prerequisites

* C++ compiler        - See supported [compilers](https://github.com/symengine/symengine/wiki/Compiler-Support)

* CMake               - with executable folder in the `PATH` variable

* libsymengine        - See build [instructions](https://github.com/symengine/symengine/wiki/Building-SymEngine)

### Installing

```toml
[dependencies]
symengine = "0.1"
```

#### features

* [`serde`](https://github.com/serde-rs/serde): serializing and deserializing

## License

The following projects are licensed under [BSD-3-Clause](https://github.com/podo-os/symengine.rs/blob/master/LICENSE).

## References

* [SymEngine](https://github.com/symengine/symengine)
* [SymEngine Ruby Wrappers](https://github.com/symengine/symengine.rb)
* [bindgen](https://github.com/rust-lang/rust-bindgen)
* [serde](https://github.com/serde-rs/serde)
