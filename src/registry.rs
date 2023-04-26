use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use crate::config::ConfigLoader;

use crate::core::{Binding, Handler};
use crate::registry::grid_mode_handler::GridModeHandler;
use crate::registry::mb_emulation_handler::MButtonsEmulationHandler;
use crate::registry::precise_mode_handler::PreciseModeHandler;

mod precise_mode_handler;
mod mb_emulation_handler;
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
        let config = ConfigLoader::load_default();

        let all_handlers: Vec<Box<dyn Handler+Send>> = vec![
            Box::new(GridModeHandler::new(config.get_handler_config(grid_mode_handler::HANDLER_ID))), // todo hide handler id inside handler
            Box::new(MButtonsEmulationHandler::new(config.get_handler_config(mb_emulation_handler::HANDLER_ID))),
            Box::new(PreciseModeHandler::new(config.get_handler_config(precise_mode_handler::HANDLER_ID))),
        ];

        let handlers: Arc<Vec<Mutex<Box<dyn Handler + Send>>>> = Arc::new(all_handlers.into_iter()
            .filter(|handler| config.is_handler_active(&handler.get_id()))
            .map(Mutex::new)
            .collect());
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
