#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn add(a:i32, b:i32) -> i32 {
        a+b
    }
    
    fn test() -> bindings::Customer {
        bindings::Customer { id: 10, name: String::from("cc") }
    }
}

bindings::export!(Component with_types_in bindings);
