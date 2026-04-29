pub trait State: Clone + Copy + Default + PartialEq {}

pub trait Stateful {
    type S: State;

    fn state(&self) -> Self::S;
    fn set_state(&mut self, new_state: Self::S);

    fn transition_to(&mut self, new_state: Self::S) {
        if self.state() == new_state {
            return;
        }

        self.on_state_exit();
        self.set_state(new_state);
        self.on_state_enter();
    }

    fn on_state_exit(&mut self);
    fn on_state_enter(&mut self);
}
