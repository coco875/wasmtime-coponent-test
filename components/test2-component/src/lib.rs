#[allow(warnings)]
mod bindings;

use bindings::{spaghettikart::module::iactor, Guest};

struct Component;

static MOD_ID: &str = "test2";

impl iactor::Actor {
    fn init(&self, x: f32, y: f32) {
        if self.get_mod_id() != MOD_ID {
            self.outside_init(x, y);
        } else {
            Component::actor_init(self, x, y);
        }
    }
}

impl Guest for Component {
    fn init() {
        println!("init {}", MOD_ID);
        let actor = iactor::Actor::new("test1", "test");
        actor.init(0.0, 0.0);
    }

    fn actor_init(a: &iactor::Actor, x: f32, y: f32) {
        println!("actor_init {} from {}", a.get_id(), MOD_ID);
        a.set_x(x);
        a.set_y(y);
    }

    fn actor_update(a: &iactor::Actor) {
        println!("actor_update {} from {}", a.get_id(), MOD_ID);
    }

    fn actor_render(a: &iactor::Actor) {
        println!("actor_render {} from {}", a.get_id(), MOD_ID);
    }
}

bindings::export!(Component with_types_in bindings);

