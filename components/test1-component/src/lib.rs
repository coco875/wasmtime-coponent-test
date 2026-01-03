#[allow(warnings)]
mod bindings;

use bindings::{spaghettikart::module::iactor, Guest};

struct Component;

static MOD_ID: &str = "test1";

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
        let actor = iactor::Actor::new("test2", "test");
        actor.init(0.0, 0.0);
    }

    fn actor_init(a: &iactor::Actor, x: f32, y: f32) {
        if a.get_id() == "test" {
            println!("init test from {}", MOD_ID);
            a.set_x(x + 5.0);
            a.set_y(y + 5.0);
        } else {
            println!("failed to init actor");
        }
    }

    fn actor_update(a: &iactor::Actor) {
        if a.get_id() == "test" {
            println!("update test");
        } else {
            println!("failed to update actor");
        }
    }

    fn actor_render(a: &iactor::Actor) {
        if a.get_id() == "test" {
            println!("render test");
        } else {
            println!("failed to render actor");
        }
    }
}

bindings::export!(Component with_types_in bindings);
