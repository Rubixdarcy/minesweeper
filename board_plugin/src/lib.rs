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

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {

        app.register_type::<BoardOptions>();
        app.register_type::<Bomb>();
        app.register_type::<BombNeighbor>();
        app.register_type::<Uncover>();

        app.add_startup_system(systems::spawn::create_board);
        app.add_system(systems::input::input_handling);
        app.add_system(systems::uncover::trigger_event_handler);
        app.add_system(systems::uncover::uncover_tiles);

        app.add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");
    }
}
