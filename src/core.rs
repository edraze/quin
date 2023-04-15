pub enum Labels {
    Keys(String)
}

pub struct State {}

pub trait Label {
    fn get_labels(&self) -> Vec<Labels>;
}

pub trait Handler: Label {
    fn execute(&mut self, label: &Labels, state: &mut State);

    fn visit(&mut self, label: &Labels, state: &mut State) {
        self.execute(label, state);
    }
}

pub struct Executor {
    handlers: Vec<Box<dyn Handler>>,
    state: State,
}

impl Executor {
    pub fn new(handlers: Vec<Box<dyn Handler>>, state: State) -> Executor {
        Executor { handlers, state }
    }

    pub fn run(&mut self, label: &Labels) {
        self.handlers.iter_mut()
            .for_each(|handler| handler.execute(&label, &mut self.state))
    }
}
