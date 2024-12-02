# How to run

```
cargo build --target wasm32-unknown-unknown --release
sdf-viewer app --loading-passes=2 --max-voxels-side=64 url target/wasm32-unknown-unknown/release/hut.wasm
```
