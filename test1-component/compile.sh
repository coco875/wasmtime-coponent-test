cd test1-component
cargo component build
rm ../loader/test1.wasm
cp target/wasm32-wasip1/debug/rust_component.wasm ../loader/test1.wasm 