use godot::{
    classes::{Input, InputEvent, input::MouseMode},
    prelude::*,
};

use crate::{
    ball::{Ball, BallState},
    bricks::Bricks,
    paddle::{Paddle, PaddleState},
    state::{State, Stateful},
    ui::{UI, UIState},
};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Main {
    base: Base<Node>,

    state: GameState,

    #[init(node = "UI")]
    ui: OnReady<Gd<UI>>,

    #[init(node = "Ball")]
    ball: OnReady<Gd<Ball>>,

    #[init(node = "Paddle")]
    paddle: OnReady<Gd<Paddle>>,

    #[init(node = "Bricks")]
    bricks: OnReady<Gd<Bricks>>,

    #[init(val = None)]
    paused_cache: Option<GameState>,
}

#[godot_api]
impl INode for Main {
    fn ready(&mut self) {
        self.ui.hide();
        self.bricks.hide();

        self.ui
            .signals()
            .start_game()
            .connect_other(&*self, Self::on_ui_start_game);

        self.ball
            .signals()
            .ball_launch()
            .connect_other(&*self, Self::on_ball_launch);

        self.ball
            .signals()
            .ball_out_screen()
            .connect_other(&*self, Self::on_ball_out_screen);

        self.bricks
            .signals()
            .cleared()
            .connect_other(&*self, Self::on_bricks_cleared);

        self.ui
            .signals()
            .resume_game()
            .connect_other(&*self, Self::on_ui_resume_game);

        self.on_state_enter();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if (self.state == GameState::Playing || self.state == GameState::Aiming)
            && event.is_action_pressed("paused")
        {
            self.paused_cache = Some(self.state);
            self.transition_to(GameState::Paused);
        }
    }
}

impl Main {
    fn on_ui_start_game(&mut self) {
        self.transition_to(GameState::Aiming);
    }

    fn on_ui_resume_game(&mut self) {
        if let Some(last_state) = self.paused_cache {
            self.transition_to(last_state);
        }
    }

    fn on_ball_launch(&mut self) {
        self.transition_to(GameState::Playing);
    }

    fn on_ball_out_screen(&mut self) {
        self.transition_to(GameState::GameOver);
    }

    fn on_bricks_cleared(&mut self) {
        self.transition_to(GameState::Victory);
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
enum GameState {
    #[default]
    Title,
    Aiming,
    Playing,
    Paused,
    Victory,
    GameOver,
}
impl State for GameState {}

impl Stateful for Main {
    type S = GameState;

    fn on_state_enter(&mut self) {
        match self.state {
            GameState::Title => {
                self.paddle.bind_mut().transition_to(PaddleState::Frozen);
                self.ball.bind_mut().transition_to(BallState::Frozen);

                self.ui.bind_mut().transition_to(UIState::Title);
                self.ui.show();
            }
            GameState::Aiming => {
                Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

                self.paddle.bind_mut().transition_to(PaddleState::Frozen);
                self.ball.bind_mut().transition_to(BallState::Aiming);

                self.paddle.bind_mut().aiming_position();
                self.ball.bind_mut().aiming_position();

                self.bricks.bind_mut().clear_bricks();
                self.bricks.bind_mut().generate();
                self.bricks.show();

                self.ui.hide();
            }
            GameState::Playing => {
                Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

                self.paddle.bind_mut().transition_to(PaddleState::Active);
                self.ball.bind_mut().transition_to(BallState::Active);

                self.ui.hide();
            }
            GameState::Paused => {
                self.paddle.bind_mut().transition_to(PaddleState::Frozen);
                self.ball.bind_mut().transition_to(BallState::Frozen);

                self.ui.bind_mut().transition_to(UIState::Paused);
                self.ui.show();
            }
            GameState::Victory => {
                self.paddle.bind_mut().transition_to(PaddleState::Frozen);
                self.ball.bind_mut().transition_to(BallState::Frozen);

                self.ui.bind_mut().transition_to(UIState::Victory);
                self.ui.show();
            }
            GameState::GameOver => {
                self.paddle.bind_mut().transition_to(PaddleState::Frozen);
                self.ball.bind_mut().transition_to(BallState::Frozen);

                self.ui.bind_mut().transition_to(UIState::GameOver);
                self.ui.show();
            }
        }
    }

    fn on_state_exit(&mut self) {
        match self.state {
            GameState::Title => {}
            GameState::Aiming => {
                Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
            }
            GameState::Playing => {
                Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
            }
            GameState::Paused => {}
            GameState::Victory => {}
            GameState::GameOver => {}
        }
    }

    fn set_state(&mut self, new_state: Self::S) {
        self.state = new_state;
    }

    fn state(&self) -> Self::S {
        self.state
    }
}
