use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, InputEvent, InputEventMouseMotion},
    prelude::*,
};

use crate::state::{State, Stateful};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Paddle {
    base: Base<CharacterBody2D>,

    state: PaddleState,

    #[init(val = 0.0)]
    x_offset: real,

    #[init(val = OnReady::manual())]
    clamp_range: OnReady<Vector2>,
}

#[godot_api]
impl ICharacterBody2D for Paddle {
    fn ready(&mut self) {
        let size = self.base().get_viewport_rect().size;
        self.clamp_range.init(Vector2::new(100.0, size.x - 100.0));

        self.on_state_enter();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.state != PaddleState::Active {
            return;
        }

        let Ok(motion) = event.try_cast::<InputEventMouseMotion>() else {
            return;
        };

        self.x_offset += motion.get_relative().x;
    }

    fn physics_process(&mut self, _delta: f32) {
        if self.state != PaddleState::Active {
            return;
        }

        let position = self.base().get_position();
        let target_x = (position.x + self.x_offset).clamp(self.clamp_range.x, self.clamp_range.y);

        self.base_mut()
            .set_position(Vector2::new(target_x, position.y));

        self.x_offset = 0.0;
    }
}

impl Paddle {
    pub fn aiming_position(&mut self) {
        self.base_mut().set_position(Vector2::new(640.0, 700.0));
    }
}

#[derive(PartialEq, Default, Clone, Copy)]
pub enum PaddleState {
    #[default]
    Frozen,
    Active,
}
impl State for PaddleState {}

impl Stateful for Paddle {
    type S = PaddleState;

    fn on_state_enter(&mut self) {
        match self.state {
            PaddleState::Active => {
                self.x_offset = 0.0;
            }
            PaddleState::Frozen => {
                self.base_mut().set_physics_process(false);
            }
        }
    }

    fn on_state_exit(&mut self) {
        match self.state {
            PaddleState::Active => {}
            PaddleState::Frozen => {
                self.base_mut().set_physics_process(true);
            }
        }
    }

    fn set_state(&mut self, new_state: Self::S) {
        self.state = new_state;
    }

    fn state(&self) -> Self::S {
        self.state
    }
}
