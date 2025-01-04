cd test2-component
cargo component build
rm ../loader/test2.wasm
cp target/wasm32-wasip1/debug/rust_component.wasm ../loader/test2.wasm 