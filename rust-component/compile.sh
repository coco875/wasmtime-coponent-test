cargo component build --release
rm ../loader/test2.wasm
cp target/wasm32-wasip1/release/rust_component.wasm ../loader/test2.wasm 