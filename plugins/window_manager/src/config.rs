use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::input_model::Key::{AltLeft, DownArrow, Escape, F, J, K, L, LeftArrow, M, O, RightArrow, SemiColon, ShiftLeft, T, UpArrow, X};
use global_input_api::input_model::Sequence;
use global_input_api::input_model::views::definition::P;

use crate::TILING_WINDOW_MANAGER_PLUGIN_NAME;

#[derive(Resource, Serialize, Deserialize, Debug, Clone, Default)]
pub struct TilingWindowManagerConfig {
    pub key_bindings: TilingWindowManagerBindings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilingWindowManagerBindings {
    pub focus_left: Vec<Sequence>,
    pub focus_right: Vec<Sequence>,
    pub focus_up: Vec<Sequence>,
    pub focus_down: Vec<Sequence>,
    pub move_left: Vec<Sequence>,
    pub move_right: Vec<Sequence>,
    pub move_up: Vec<Sequence>,
    pub move_down: Vec<Sequence>,
    pub stack_left: Vec<Sequence>,
    pub stack_right: Vec<Sequence>,
    pub stack_up: Vec<Sequence>,
    pub stack_down: Vec<Sequence>,
    pub unstack: Vec<Sequence>,
    pub toggle_maximize: Vec<Sequence>,
    pub toggle_monocle: Vec<Sequence>,
    pub toggle_float: Vec<Sequence>,
    pub minimize: Vec<Sequence>,
    pub close: Vec<Sequence>,
}

impl Default for TilingWindowManagerBindings {
    fn default() -> Self {
        Self {
            focus_left: vec![
                (AltLeft, P(J).into()).into()
            ],
            focus_right: vec![
                (AltLeft, P(SemiColon).into()).into()
            ],
            focus_up: vec![
                (AltLeft, P(L).into()).into()
            ],
            focus_down: vec![
                (AltLeft, P(K).into()).into()
            ],
            move_left: vec![
                (vec![AltLeft, ShiftLeft], P(J).into()).into(),
            ],
            move_right: vec![
                (vec![AltLeft, ShiftLeft], P(SemiColon).into()).into(),
            ],
            move_up: vec![
                (vec![AltLeft, ShiftLeft], P(L).into()).into(),
            ],
            move_down: vec![
                (vec![AltLeft, ShiftLeft], P(K).into()).into(),
            ],
            stack_left: vec![
                (AltLeft, P(LeftArrow).into()).into()
            ],
            stack_right: vec![
                (AltLeft, P(RightArrow).into()).into()
            ],
            stack_up: vec![
                (AltLeft, P(UpArrow).into()).into()
            ],
            stack_down: vec![
                (AltLeft, P(DownArrow).into()).into()
            ],
            unstack: vec![
                (AltLeft, P(Escape).into()).into()
            ],
            toggle_maximize: vec![
                (AltLeft, P(O).into()).into()
            ],
            toggle_monocle: vec![
                (AltLeft, P(T).into()).into()
            ],
            toggle_float: vec![
                (AltLeft, P(F).into()).into()
            ],
            minimize: vec![
                (AltLeft, P(M).into()).into()
            ],
            close: vec![
                (AltLeft, P(X).into()).into()
            ],
        }
    }
}

impl Config for TilingWindowManagerConfig {
    fn name() -> String {
        TILING_WINDOW_MANAGER_PLUGIN_NAME.to_string()
    }
}
