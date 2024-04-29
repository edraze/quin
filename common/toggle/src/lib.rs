use std::marker::PhantomData;
use bevy::app::App;
use bevy::prelude::{Event, EventReader, Events, EventWriter, Res, ResMut, Resource, Update};

pub fn add_toggle<A: Event + Clone, D: Event + Clone>(app: &mut App) {
    if !app.world.contains_resource::<Events<A>>() {
        app.add_event::<A>();
    }
    if !app.world.contains_resource::<Events<D>>() {
        app.add_event::<D>();
    }
    app.init_resource::<Toggle<A,D>>();
    app.add_systems(Update, toggle::<A, D>);
}

pub fn add_toggle_event<A: Event + Clone, D: Event + Clone, E: Event + Clone>(app: &mut App) {
    if !app.world.contains_resource::<Events<E>>() {
        app.add_event::<E>();
    }
    app.add_event::<Active<E>>();
    app.add_event::<Inactive<E>>();
    app.add_systems(Update, mapper::<A,D,E>);
}

fn toggle<A: Event + Clone, D: Event + Clone>(mut activate_events: EventReader<A>, mut deactivate_events: EventReader<D>,
                                              mut toggle: ResMut<Toggle<A, D>>) {
    if activate_events.read().count() > 0 {
        toggle.is_active = true;
    }
    if deactivate_events.read().count() > 0 {
        toggle.is_active = false;
    }
}

fn mapper<A: Event + Clone, D: Event + Clone, E: Event + Clone>(mut in_events: EventReader<E>,
                                                                toggle: Res<Toggle<A, D>>,
                                                                mut active_events_writer: EventWriter<Active<E>>,
                                                                mut inactive_events_writer: EventWriter<Inactive<E>>) {
    for event in in_events.read().cloned() {
        if toggle.is_active {
            active_events_writer.send(Active(event));
        } else {
            inactive_events_writer.send(Inactive(event));
        }
    }
}

#[derive(Resource)]
struct Toggle<A: Event + Clone, D: Event + Clone> {
    _activation_event: PhantomData<A>,
    _deactivation_event: PhantomData<D>,
    is_active: bool,
}

impl <A: Event + Clone, D: Event + Clone>Default for Toggle<A,D> {
    fn default() -> Self {
        Self{
            _activation_event: PhantomData,
            _deactivation_event: PhantomData,
            is_active: false
        }
    }
}

#[derive(Event)]
pub struct Active<E: Event>(E);

#[derive(Event)]
pub struct Inactive<E: Event>(E);