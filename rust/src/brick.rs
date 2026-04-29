use godot::{
    classes::{IStaticBody2D, StaticBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=StaticBody2D)]
pub struct Brick {
    base: Base<StaticBody2D>,
}

#[godot_api]
impl IStaticBody2D for Brick {
    fn ready(&mut self) {
        self.signals().hited().connect_self(|this| {
            this.base_mut().queue_free();
        });
    }
}

#[godot_api]
impl Brick {
    #[signal]
    pub fn hited();
}
