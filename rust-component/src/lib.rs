#[allow(warnings)]
mod bindings;

use bindings::{init_actor, Guest};

struct Component;

static MOD_ID: &str = "test2";

impl bindings::Actor {
    fn init(&self) {
        init_actor(self);
    }
}

impl Guest for Component {
    fn init() {
        println!("init");
        let actor = bindings::Actor {
            mod_id: "test2".to_owned(),
            id: "test".to_owned(),
            x: 0.0,
            y: 0.0,
        };
        actor.init();
    }

    fn actor_update(a: bindings::Actor) -> bool {
        if a.id == "test" {
            println!("update test");
        } else {
            return false;
        }
        return true;
    }

    fn actor_render(a: bindings::Actor) -> bool {
        if a.id == "test" {
            println!("render test");
        } else {
            return false;
        }
        return true;
    }

    fn actor_init(mut a: bindings::Actor) -> bool {
        if a.id == "test" {
            println!("init test");
            a.x = 10.0;
            a.y = 10.0;
        } else {
            return false;
        }
        return true;
    }
}

bindings::export!(Component with_types_in bindings);
