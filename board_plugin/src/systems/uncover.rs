use bevy::{prelude::*, log};

use crate::{events::TileTriggerEvent, resources::Board, components::{Uncover, BombNeighbor, Coordinates, Bomb}};

pub fn trigger_event_handler(
    mut cmd: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for event in tile_trigger_evr.iter() {
        if let Some(tile_cover_entity) = board.get_covered_tile(&event.0) {
            cmd.entity(*tile_cover_entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut cmd: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        cmd.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };
        match board.try_uncover_tile(coords) {
            None => log::debug!("Tried to uncover an already uncovered tile"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
        }
        if bomb.is_some() {
            log::info!("Boom !");
            // TODO: Add explosion event
        }
        else if bomb_counter.is_none() {
            for &entity in board.adjacent_covered_tiles(coords) {
                cmd.entity(entity).insert(Uncover);
            }
        }
    }
}