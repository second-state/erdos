# Multi Nodes Demo

```console
rustup default nightly

cd wasm/add
rustwasmc build

cd ../..
cargo build
./run_main.sh
```

# Tensorflow Demo

### Prerequisite:
Download prebuilt WasmEdge-tensorflow-tools first: [https://github.com/second-state/WasmEdge-tensorflow-tools](https://github.com/second-state/WasmEdge-tensorflow-tools)

```console
rustup default nightly
cd wasm/food_classify
rustwasmc build

cd ../..
cargo run --bin image_classify
```
