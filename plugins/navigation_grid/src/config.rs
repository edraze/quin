use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::input_model::definition::{P, R};
use global_input_api::input_model::Key::{AltRight, Escape};
use input_sequence_api::Sequence;

use crate::plugin::NAVIGATION_GRID_PLUGIN_NAME;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct NavigationGridConfig {
    pub allowed_label_key: String,
    pub key_bindings: NavigationGridBindings,
}

impl Default for NavigationGridConfig {
    fn default() -> Self {
        Self {
            allowed_label_key: "qwertyuiopasdfghjklzxcvbnm".to_string(), // todo qQwWeErRtTyYuUiIoOpPaAsSdDfFgGhHjJkKlLzZxXcCvVbBnNmM
            key_bindings: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavigationGridBindings {
    pub activate: Vec<Sequence>,
    pub deactivate: Vec<Sequence>,
}

impl Default for NavigationGridBindings {
    fn default() -> Self {
        Self {
            activate: vec![
                Sequence::new(vec![
                    P(AltRight).into(),
                    R(AltRight).into(),
                ])],
            deactivate: vec![
                Sequence::new(vec![
                    P(Escape).into()
                ])],
        }
    }
}

impl Config for NavigationGridConfig {
    fn name() -> String {
        NAVIGATION_GRID_PLUGIN_NAME.to_string()
    }
}
