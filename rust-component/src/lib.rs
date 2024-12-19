#[allow(warnings)]
mod bindings;

use bindings::{register, Guest, Simple};

struct ItemA {
    name: u32,
}

struct Component;

impl Guest for Component {
    /// Say hello!
    fn add(a:i32, b:i32) -> i32 {
        a+b
    }
    
    fn test() -> bindings::Customer {
        bindings::Customer { id: 10, name: String::from("cc") }
    }
    
    fn init() {
        register(Simple);
    }
}

bindings::export!(Component with_types_in bindings);
