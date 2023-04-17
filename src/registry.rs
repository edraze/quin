use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use grid_mode_handler::GridModeHandler;

use crate::core::{Binding, Handler};

mod grid_mode_handler;

pub fn get_handlers() -> Arc<Vec<Mutex<Box<dyn Handler + Send>>>> {
    REGISTRY.lock()
        .expect("Registry already locked")
        .get_handlers()
}

pub fn get_bindings() -> Vec<Binding> {
    REGISTRY.lock()
        .expect("Registry already locked")
        .get_bindings()
}

static REGISTRY: Lazy<Mutex<Registry>> = Lazy::new(|| Mutex::new(Registry::default()));

struct Registry {
    handlers: Arc<Vec<Mutex<Box<dyn Handler + Send>>>>,
}

impl Default for Registry {
    fn default() -> Self {
        let handlers: Arc<Vec<Mutex<Box<dyn Handler + Send>>>> = Arc::new(vec![
            Mutex::new(Box::<GridModeHandler>::default())
        ]);
        Registry { handlers }
    }
}

impl Registry {
    fn get_handlers(&self) -> Arc<Vec<Mutex<Box<dyn Handler + Send>>>> {
        Arc::clone(&self.handlers)
    }

    fn get_bindings(&self) -> Vec<Binding> {
        self.handlers.iter()
            .flat_map(|handler| handler.lock()
                .expect("Handler already locked").
                get_bindings())
            .collect()
    }
}
