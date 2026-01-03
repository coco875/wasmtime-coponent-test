mod state;
use component::{Resource, ResourceTableError};
use spaghettikart::module::iactor::{self, HostActor};
use state::MyState;
use std::{collections::HashMap, error::Error, hash::BuildHasherDefault};
use twox_hash::XxHash64;
use wasmtime::*;

#[derive(Debug)]
pub struct MyActor {
    mod_id: String,
    id: String,
    x: f32,
    y: f32,
}

component::bindgen!({
    path: "wit",

    with: {
        "spaghettikart:module/iactor/actor": MyActor,
    },

    trappable_imports: true,
});

static mut LOADER: Option<Loader> = None;

pub struct Loader {
    mods: HashMap<String, ModWorld, BuildHasherDefault<XxHash64>>,
    store: Store<MyState>,
}

impl Loader {
    pub fn init(engine: &Engine) {
        let wasi_ctx = MyState::new();
        let store = Store::new(engine, wasi_ctx);
        unsafe {
            LOADER = Some(Loader {
                mods: HashMap::default(),
                store,
            })
        };
    }

    pub fn load(
        &mut self,
        path: &str,
        engine: &Engine,
        linker: &component::Linker<MyState>,
    ) -> Result<(), Box<dyn Error>> {
        println!("load {}", path);
        let bytes = std::fs::read(path.to_owned() + ".wasm")?;
        let component = component::Component::new(engine, bytes)?;
        let instance = ModWorld::instantiate(&mut self.store, &component, linker)?;
        self.mods.insert(path.to_string(), instance.into());
        Ok(())
    }

    pub fn get_loader() -> &'static mut Loader {
        unsafe { LOADER.as_mut().unwrap() }
    }

    pub fn get_store() -> &'static mut Store<MyState> {
        unsafe { &mut LOADER.as_mut().unwrap().store }
    }

    pub fn new_actor(
        &mut self,
        mod_id: String,
        id: String,
    ) -> Result<Resource<Actor>, wasmtime::Error> {
        let actor = self.store.data_mut().new(mod_id, id)?;
        let struct_actor = self.store.data_mut().table.get_mut(&actor)?;
        self.mods[&struct_actor.mod_id].call_actor_init(
            &mut self.store,
            Resource::new_borrow(actor.rep()),
            0.0,
            0.0,
        )?;
        Ok(actor)
    }

    pub fn actor_struct(
        &mut self,
        actor: Resource<Actor>,
    ) -> Result<&mut MyActor, ResourceTableError> {
        self.store.data_mut().table.get_mut(&actor)
    }
}

impl ModWorldImports for MyState {
    fn init_actor(&mut self, actor: Resource<Actor>) -> std::result::Result<(), wasmtime::Error> {
        debug_assert!(!actor.owned());
        let loader = Loader::get_loader();
        let store = Loader::get_store();
        let struct_actor = self.table.get_mut(&actor)?;
        loader.mods[&struct_actor.mod_id].call_actor_init(store, actor, 0.0, 0.0)?;
        Ok(())
    }
}

impl HostActor for MyState {
    fn new(&mut self, mod_id: String, id: String) -> Result<Resource<Actor>, wasmtime::Error> {
        let id = self.table.push(Actor {
            mod_id,
            id,
            x: 0.0,
            y: 0.0,
        })?;
        Ok(id)
    }

    fn outside_init(
        &mut self,
        actor: Resource<Actor>,
        x: f32,
        y: f32,
    ) -> Result<(), wasmtime::Error> {
        debug_assert!(!actor.owned());
        let loader = Loader::get_loader();
        let store = Loader::get_store();
        let struct_actor = self.table.get_mut(&actor)?;
        println!("outside_init");
        loader.mods[&struct_actor.mod_id].call_actor_init(store, actor, x, y)?;
        Ok(())
    }

    fn update(&mut self, actor: Resource<Actor>) -> Result<(), wasmtime::Error> {
        debug_assert!(!actor.owned());
        let loader = Loader::get_loader();
        let store = Loader::get_store();
        let struct_actor = self.table.get_mut(&actor)?;
        loader.mods[&struct_actor.mod_id].call_actor_update(store, actor)?;
        Ok(())
    }
    fn render(&mut self, actor: Resource<Actor>) -> Result<(), wasmtime::Error> {
        debug_assert!(!actor.owned());
        let loader = Loader::get_loader();
        let store = Loader::get_store();
        let struct_actor = self.table.get_mut(&actor)?;
        loader.mods[&struct_actor.mod_id].call_actor_render(store, actor)?;
        Ok(())
    }
    fn get_mod_id(&mut self, actor: Resource<Actor>) -> Result<String, wasmtime::Error> {
        debug_assert!(!actor.owned());
        let struct_actor = self.table.get_mut(&actor)?;
        Ok(struct_actor.mod_id.clone())
    }
    fn get_id(&mut self, actor: Resource<Actor>) -> Result<String, wasmtime::Error> {
        debug_assert!(!actor.owned());
        let struct_actor = self.table.get_mut(&actor)?;
        Ok(struct_actor.id.clone())
    }
    fn get_x(&mut self, actor: Resource<Actor>) -> Result<f32, wasmtime::Error> {
        debug_assert!(!actor.owned());
        let struct_actor = self.table.get_mut(&actor)?;
        Ok(struct_actor.x)
    }
    fn set_x(&mut self, actor: Resource<Actor>, x: f32) -> Result<(), wasmtime::Error> {
        debug_assert!(!actor.owned());
        let struct_actor = self.table.get_mut(&actor)?;
        struct_actor.x = x;
        Ok(())
    }
    fn get_y(&mut self, actor: Resource<Actor>) -> Result<f32, wasmtime::Error> {
        debug_assert!(!actor.owned());
        let struct_actor = self.table.get_mut(&actor)?;
        Ok(struct_actor.y)
    }
    fn set_y(&mut self, actor: Resource<Actor>, y: f32) -> Result<(), wasmtime::Error> {
        debug_assert!(!actor.owned());
        let struct_actor = self.table.get_mut(&actor)?;
        struct_actor.y = y;
        Ok(())
    }
    fn drop(&mut self, actor: Resource<Actor>) -> Result<(), wasmtime::Error> {
        debug_assert!(actor.owned());
        let _actor = self.table.delete(actor)?;
        // ... custom destruction logic here if necessary, otherwise
        // a `Drop for MyLogger` would also work.
        Ok(())
    }
}

impl iactor::Host for MyState {}

fn main() -> Result<(), Box<dyn Error>> {
    // An engine stores and configures global compilation settings like
    // optimization level, enabled wasm features, etc.
    let mut config = Config::new();
    config.debug_info(true);
    let engine = Engine::new(&config)?;
    let mut linker = component::Linker::<MyState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)?;
    ModWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    Loader::init(&engine);
    let loader = Loader::get_loader();

    println!("=== Discovering wasm components ===\n");

    // Discover all .wasm files in current directory
    let mut components: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "wasm" {
                if let Some(stem) = path.file_stem() {
                    if let Some(name) = stem.to_str() {
                        components.push(name.to_string());
                    }
                }
            }
        }
    }

    // Sort for consistent ordering
    components.sort();

    println!("Found {} wasm files: {:?}\n", components.len(), components);

    println!("=== Loading all components ===\n");

    // Load all components
    for component in &components {
        match loader.load(component, &engine, &linker) {
            Ok(_) => println!("✓ Loaded: {}", component),
            Err(e) => println!("✗ Failed to load {}: {}", component, e),
        }
    }

    println!("\n=== Testing components ===\n");

    // Test each loaded component
    for component in &components {
        if !loader.mods.contains_key(component) {
            println!("⚠ Skipping {} (not loaded)", component);
            continue;
        }

        println!("--- Testing {} ---", component);
        
        // Call init on the component
        let store = Loader::get_store();
        match loader.mods[component].call_init(&mut *store) {
            Ok(_) => println!("  ✓ init() called successfully"),
            Err(e) => println!("  ✗ init() failed: {}", e),
        }

        // Create an actor from this component
        let actor_id = format!("actor-from-{}", component);
        match loader.new_actor(component.to_string(), actor_id.clone()) {
            Ok(actor) => {
                println!("  ✓ Actor created: {}", actor_id);
                
                // Get actor struct and display info
                match loader.actor_struct(actor) {
                    Ok(actor_struct) => {
                        println!("    Actor details: {:?}", actor_struct);
                    }
                    Err(e) => println!("    ✗ Failed to get actor struct: {}", e),
                }
            }
            Err(e) => println!("  ✗ Failed to create actor: {}", e),
        }

        println!();
    }

    println!("=== All tests completed ===");
    Ok(())
}

