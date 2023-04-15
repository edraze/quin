use grid_mode_handler::GridModeHandler;

use crate::core::{Handler, Labels};

mod grid_mode_handler;

// todo singleton inside this module
pub struct Registry {
    handlers: Vec<Box<dyn Handler>>,
}

impl Default for Registry {
    fn default() -> Self {
        let handlers: Vec<Box<dyn Handler>> = vec![
            Box::<GridModeHandler>::default()
        ];
        Registry { handlers }
    }
}

impl Registry {
    pub fn get_handlers(self) -> Vec<Box<dyn Handler>> {
        self.handlers
    }

    pub fn get_labels(&self) -> Vec<Labels> {
        self.handlers.iter()
            .flat_map(|handler| handler.get_labels())
            .collect()
    }
}
