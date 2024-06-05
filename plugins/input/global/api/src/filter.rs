use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use input_model::filter::InputFilter;

#[derive(Event, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputFilterEvent(pub InputFilter);
