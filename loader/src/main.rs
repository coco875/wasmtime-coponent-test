use std::error::Error;
use component::ResourceTable;
// use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;
use wasmtime_wasi::{preview1::WasiP1Ctx, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

struct MyState {
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiHttpView for MyState {
    fn ctx(&mut self) -> &mut WasiHttpCtx { &mut self.http_ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

impl MyState {
    fn new() -> MyState {
        let mut wasi = WasiCtxBuilder::new();

        MyState {
            ctx: wasi.build(),
            table: ResourceTable::new(),
            http_ctx: WasiHttpCtx::new(),
        }
    }
}

component::bindgen!();

fn main() -> Result<(), Box<dyn Error>> {
    // An engine stores and configures global compilation settings like
    // optimization level, enabled wasm features, etc.
    let mut config = Config::new();
    let engine = Engine::new(&config)?;
    let mut linker = component::Linker::<MyState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)?;

    let mut wasi = WasiCtxBuilder::new();

    let wasi_ctx = MyState {
            ctx: wasi.build(),
            table: ResourceTable::new(),
            http_ctx: WasiHttpCtx::new(),
        };
    let mut store = Store::new(&engine, wasi_ctx);

    println!("load component");
    let bytes = std::fs::read("js.wasm")?;
    let component = component::Component::new(&engine, bytes)?;

    println!("initialise component");
    let instance = Example::instantiate(&mut store, &component, &linker)?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    let mut result = instance.call_add(&mut store, 1, 2)?;
    println!("Answer: {:?}", result);

    let mut result = instance.call_test(&mut store)?;
    println!("Answer: {:?}", result);
    Ok(())
}