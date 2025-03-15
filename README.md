# Brique demo wasm

## How to compile 

```sh 
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web
```
