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

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone, States, Reflect)]
pub enum BoardState {
    Active,
    #[default]
    Inactive,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {

        app.register_type::<BoardOptions>();
        app.register_type::<Bomb>();
        app.register_type::<BombNeighbor>();
        app.register_type::<Uncover>();

        app.add_state::<BoardState>();

        app.add_system(systems::spawn::create_board.in_schedule(OnEnter(BoardState::Active)));
        app.add_system(systems::spawn::despawn_board.in_schedule(OnExit(BoardState::Active)));

        let active = || OnUpdate(BoardState::Active);

        app.add_system(systems::input::input_handling.in_set(active()));
        app.add_system(systems::uncover::trigger_event_handler.in_set(active()));
        app.add_system(systems::uncover::uncover_tiles.in_set(active()));

        app.add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");
    }
}
