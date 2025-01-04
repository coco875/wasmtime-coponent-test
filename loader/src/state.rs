use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

pub struct MyState {
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
    pub table: ResourceTable,
}

impl WasiHttpView for MyState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl MyState {
    pub fn new() -> Self {
        let mut wasi = WasiCtxBuilder::new();

        wasi.inherit_stdio();

        MyState {
            ctx: wasi.build(),
            table: ResourceTable::new(),
            http_ctx: WasiHttpCtx::new(),
        }
    }
}
