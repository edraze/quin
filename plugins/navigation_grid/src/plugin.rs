use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{IntoSystemConfigs, Update};
use itertools::Itertools;

use global_input_api::input_model::Sequence;
use input_sequence_api::SequencesToEvent;
use input_sequence_plugin::listen_sequences;
use keyboard_layout::Layout;

use crate::config::NavigationGridConfig;
use crate::events::{ActivateMainGrid, ActivateNavigationGrid, ActivateSubGrid, DeactivateMainGrid, DeactivateNavigationGrid, DeactivateSubGrid, NavigateToLabel, NavigateToSubLabel, UpdateSubGridPosition};
use crate::systems::{activate_main_grid_system, activate_plugin_system, activate_sub_grid_system, deactivate_main_grid_system, deactivate_plugin_system, deactivate_sub_grid_system, init_labels_system, navigate_to_label_system, navigate_to_sub_label_system, update_sub_grid_position};

pub(crate) const NAVIGATION_GRID_PLUGIN_NAME: &str = "navigation_grid";

pub struct NavigationGridPlugin;

impl Plugin for NavigationGridPlugin {
    fn build(&self, app: &mut App) {
        let config = config_loader::load::<NavigationGridConfig>();
        app.insert_resource(config.clone());
        app.add_event::<UpdateSubGridPosition>();

        let activation_binding: SequencesToEvent<_> = (config.key_bindings.activate.clone(), ActivateNavigationGrid).into();
        let deactivation_binding: SequencesToEvent<_> = (config.key_bindings.deactivate.clone(), DeactivateNavigationGrid).into();
        listen_sequences(app, activation_binding);
        listen_sequences(app, deactivation_binding);

        let label_keys: Vec<String> = config.allowed_label_key.chars()
            .permutations(2)
            .map(String::from_iter)
            .collect();

        build_main_layout(app, &label_keys);
        build_sub_layout(app, &label_keys);

        app.add_systems(Startup, init_labels_system);
        app.add_systems(Update, activate_plugin_system);
        app.add_systems(Update, activate_main_grid_system.after(activate_plugin_system));
        app.add_systems(Update, navigate_to_label_system.after(activate_main_grid_system));
        app.add_systems(Update, update_sub_grid_position.after(navigate_to_label_system));
        app.add_systems(Update, deactivate_main_grid_system.after(update_sub_grid_position));
        app.add_systems(Update, activate_sub_grid_system.after(deactivate_main_grid_system));
        app.add_systems(Update, navigate_to_sub_label_system.after(activate_sub_grid_system));
        app.add_systems(Update, deactivate_sub_grid_system.after(navigate_to_sub_label_system));
        app.add_systems(Update, deactivate_plugin_system.after(deactivate_sub_grid_system));
    }

    fn name(&self) -> &str {
        NAVIGATION_GRID_PLUGIN_NAME
    }
}

fn build_main_layout(app: &mut App, labels: &Vec<String>) {
    let mut grid_layout = Layout::<ActivateMainGrid, DeactivateMainGrid>::new(app);
    for label in labels {
        let sequence = labels_to_sequences(label);
        if let Some(sequence) = sequence {
            let event = NavigateToLabel::new(label);
            grid_layout = grid_layout.bind((sequence, event));
        }
    }
}

fn build_sub_layout(app: &mut App, labels: &Vec<String>) {
    let mut sub_grid_layout = Layout::<ActivateSubGrid, DeactivateSubGrid>::new(app);
    for label in labels {
        let sequence = labels_to_sequences(label);
        if let Some(sequence) = sequence {
            let event = NavigateToSubLabel::new(label);
            sub_grid_layout = sub_grid_layout.bind((sequence, event));
        }
    }
}

fn labels_to_sequences(label: &str) -> Option<Sequence> {
    let input_events: Vec<_> = label.chars()
        .map(|letter| letter.to_string())
        .flat_map(|letter| Sequence::try_from(letter.as_str()))
        .flat_map(|sequence| sequence.input_events)
        .collect();
    if input_events.is_empty() {
        None
    } else {
        Some(Sequence::new(input_events))
    }
}
