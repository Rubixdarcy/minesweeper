use bevy::prelude::*;
use bevy::log;
use resources::BoardOptions;

use crate::components::Bomb;
use crate::components::BombNeighbor;
use crate::components::Uncover;
use crate::events::TileTriggerEvent;

mod bounds;
mod components;
pub mod resources;
mod systems;
mod events;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash, SystemSet)]
enum Set {
    Running,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {

        app.register_type::<BoardOptions>();
        app.register_type::<Bomb>();
        app.register_type::<BombNeighbor>();
        app.register_type::<Uncover>();

        app.configure_set(Set::Running.in_set(OnUpdate(self.running_state.clone())));        

        app.add_system(systems::spawn::create_board.in_schedule(OnEnter(self.running_state.clone())));
        app.add_system(systems::spawn::despawn_board.in_schedule(OnExit(self.running_state.clone())));

        app.add_system(systems::input::input_handling.in_set(Set::Running));
        app.add_system(systems::uncover::trigger_event_handler.in_set(Set::Running));
        app.add_system(systems::uncover::uncover_tiles.in_set(Set::Running));

        app.add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");
    }
}
