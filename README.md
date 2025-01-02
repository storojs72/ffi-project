# ffi-project

This is an example of calling Binius from Haskell environment via FFI.

Bindings are auto-generated using [cargo-cabal](https://github.com/yvan-sraka/cargo-cabal) project.

Repository contains simple Rust library (in `binius-ffi/src/lib.rs`) where `prove` function is defined and
exposed to Haskell environment via FFI. The `prove` function contains simple yet end-to-end Binius proving example
of adding two `u32` values.

In `binius-ffi/src/BiniusFfi.hs`, which is auto-generated, one can find Haskell bindings.
In `test/app/Main.hs` one can find example of invocation exposed `prove` function.

Once Rust library is modified, it has to be compiled and then,
bindings can be regenerated on the fly (while Haskell code compiling and linking)
just while running Haskell tests:
```
cd binius-ffi
cargo +nightly build
cabal run test
```

Note that Rust library should be available either in `binius-ffi/target/release` or `binius-ffi/target/debug`.
