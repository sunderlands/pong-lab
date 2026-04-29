use std::f32::consts::PI;

use godot::{
    classes::{
        IRigidBody2D, InputEvent, InputEventMouseButton, InputEventMouseMotion,
        PhysicsDirectBodyState2D, PhysicsServer2D, RigidBody2D, VisibleOnScreenNotifier2D,
        physics_server_2d::BodyState,
    },
    global::MouseButton,
    prelude::*,
};

use crate::{
    brick::Brick,
    state::{State, Stateful},
};

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub struct Ball {
    base: Base<RigidBody2D>,

    state: BallState,

    #[init(node = "Arrow")]
    arrow: OnReady<Gd<Node2D>>,

    aim_offset: real,

    #[init(node = "VisibleOnScreenNotifier2D")]
    notifier: OnReady<Gd<VisibleOnScreenNotifier2D>>,

    #[init(val = None)]
    direction_cache: Option<Vector2>,
}

#[godot_api]
impl IRigidBody2D for Ball {
    fn ready(&mut self) {
        self.arrow.hide();

        self.signals()
            .ball_launch()
            .connect_self(Self::on_ball_launch);

        self.notifier
            .signals()
            .screen_exited()
            .connect_other(&*self, Self::on_screen_exited);

        self.signals().body_entered().connect(Self::on_body_entered);

        self.on_state_enter();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.state != BallState::Aiming {
            return;
        }

        match event.try_cast::<InputEventMouseMotion>() {
            Ok(motion) => {
                self.aim_offset = (self.aim_offset + motion.get_relative().x * 0.005)
                    .clamp(-Self::OFFSET_CLAMP, Self::OFFSET_CLAMP);
            }
            Err(event) => match event.try_cast::<InputEventMouseButton>() {
                Ok(button) => {
                    if button.get_button_index() == MouseButton::LEFT {
                        self.signals().ball_launch().emit();
                    }
                }
                _ => {}
            },
        }
    }

    fn integrate_forces(&mut self, state: Option<Gd<PhysicsDirectBodyState2D>>) {
        let Some(mut state) = state else {
            return;
        };

        if self.state != BallState::Active {
            return;
        }

        let velocity = state.get_linear_velocity();

        let speed = velocity.length();
        if (speed - Self::SPEED).abs() > Self::EPSILON {
            state.set_linear_velocity(velocity.normalized() * Self::SPEED);
        }
    }

    fn process(&mut self, _delta: f32) {
        if let BallState::Aiming = self.state {
            self.arrow.set_rotation(self.aim_offset);
        }
    }
}

#[godot_api]
impl Ball {
    const OFFSET_CLAMP: real = PI / 3.0;
    const EPSILON: real = 0.001;
    const SPEED: real = 800.0;

    #[signal]
    pub fn ball_launch();

    #[signal]
    pub fn ball_out_screen();

    fn on_ball_launch(&mut self) {
        self.transition_to(BallState::Active);
    }

    fn on_screen_exited(&mut self) {
        self.transition_to(BallState::Frozen);
        self.signals().ball_out_screen().emit();
    }

    fn on_body_entered(node: Gd<Node>) {
        let Ok(brick) = node.try_cast::<Brick>() else {
            return;
        };

        brick.signals().hitted().emit();
    }

    pub fn aiming_position(&mut self) {
        let position = Vector2::new(640.0, 680.0);

        self.base_mut().set_global_position(position);

        let mut t = self.base().get_transform();
        t.origin = position;

        PhysicsServer2D::singleton().body_set_state(
            self.base().get_rid(),
            BodyState::TRANSFORM,
            &t.to_variant(),
        );

        self.base_mut().reset_physics_interpolation();
    }
}

#[derive(PartialEq, Default, Clone, Copy)]
pub enum BallState {
    #[default]
    Frozen,
    Aiming,
    Active,
}

impl State for BallState {}

impl Stateful for Ball {
    type S = BallState;

    fn on_state_enter(&mut self) {
        match self.state {
            BallState::Frozen => {
                let mut base_mut = self.base_mut();
                base_mut.call_deferred("set_sleeping", &[true.to_variant()]);
                base_mut.call_deferred("set_freeze_enabled", &[true.to_variant()]);
            }
            BallState::Aiming => {
                self.direction_cache = None;
                self.arrow.show();
            }
            BallState::Active => {
                let direction_cache = self.direction_cache.take();

                match direction_cache {
                    Some(direction) => {
                        let new_velocity = direction * Self::SPEED;
                        self.base_mut().set_linear_velocity(new_velocity);
                    }
                    None => {
                        let new_velocity =
                            Vector2::UP.rotated(std::mem::take(&mut self.aim_offset)) * Self::SPEED;
                        self.base_mut().set_linear_velocity(new_velocity);
                    }
                }

                let mut base_mut = self.base_mut();
                base_mut.set_sleeping(false);
                base_mut.set_freeze_enabled(false);
            }
        }
    }

    fn on_state_exit(&mut self) {
        match self.state {
            BallState::Frozen => {}
            BallState::Aiming => {
                self.arrow.hide();
            }
            BallState::Active => {
                let velocity = self.base().get_linear_velocity();
                if velocity == Vector2::ZERO {
                    return;
                }
                self.direction_cache = Some(velocity.normalized());

                self.base_mut()
                    .call_deferred("set_linear_velocity", &[Vector2::ZERO.to_variant()]);
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
