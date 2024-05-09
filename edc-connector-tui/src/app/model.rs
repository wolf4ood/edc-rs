#[derive(Debug, Default)]
pub struct Model {
    pub(crate) counter: i32,
    pub(crate) running_state: RunningState,
}

impl Model {
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
