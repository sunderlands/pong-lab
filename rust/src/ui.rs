use std::collections::HashMap;

use godot::{
    classes::{Button, CanvasLayer, ICanvasLayer, Label},
    prelude::*,
};

use crate::state::{State, Stateful};

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct UI {
    base: Base<CanvasLayer>,

    state: UIState,

    #[init(val = Self::contents())]
    contents: HashMap<UIState, Content>,

    #[init(node = "Label")]
    label: OnReady<Gd<Label>>,

    #[init(node = "Button")]
    button: OnReady<Gd<Button>>,
}

#[godot_api]
impl ICanvasLayer for UI {
    fn ready(&mut self) {
        self.button
            .signals()
            .button_up()
            .connect_other(&*self, Self::on_button_up);

        self.on_state_enter();
    }
}

#[godot_api]
impl UI {
    #[signal]
    pub fn start_game();

    #[signal]
    pub fn resume_game();

    fn contents() -> HashMap<UIState, Content> {
        let mut map = HashMap::new();
        map.insert(UIState::Title, ("Pong", "Play").into());
        map.insert(UIState::Paused, ("Paused", "Resume").into());
        map.insert(UIState::Victory, ("Win", "Replay").into());
        map.insert(UIState::GameOver, ("Game Over", "Replay").into());

        map
    }

    fn on_button_up(&mut self) {
        match self.state {
            UIState::Paused => self.signals().resume_game().emit(),
            _ => self.signals().start_game().emit(),
        }
    }
}

struct Content {
    label: &'static str,
    button: &'static str,
}
impl From<(&'static str, &'static str)> for Content {
    fn from(value: (&'static str, &'static str)) -> Self {
        Self {
            label: value.0,
            button: value.1,
        }
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub enum UIState {
    #[default]
    Title,
    Paused,
    Victory,
    GameOver,
}
impl State for UIState {}

impl Stateful for UI {
    type S = UIState;

    fn on_state_enter(&mut self) {
        if let Some(content) = self.contents.get(&self.state) {
            self.label.set_text(content.label);
            self.button.set_text(content.button);
        }
    }

    fn on_state_exit(&mut self) {}

    fn set_state(&mut self, new_state: Self::S) {
        self.state = new_state;
    }

    fn state(&self) -> Self::S {
        self.state
    }
}
