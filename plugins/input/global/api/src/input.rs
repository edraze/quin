use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use input_model::Input;

#[derive(Event, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputEvent(pub Input);