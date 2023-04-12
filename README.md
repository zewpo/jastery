
# To Build for WebAssembly

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_jastery --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/jastery.wasm
cargo run --bin post_wasm_build --features post_wasm_build
```

