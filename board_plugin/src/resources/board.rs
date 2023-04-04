use bevy::{window::Window, prelude::{Vec2, Resource, Entity}, utils::HashMap, log};

use crate::{bounds::Bounds2, components::Coordinates};

use super::tilemap::TileMap;


#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub entity: Entity,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub marked_tiles: Vec<Coordinates>,
}

impl Board {

    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        if !self.bounds.in_bounds(position) {
            return None;
        }

        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    pub fn get_covered_tile(&self, coords: &Coordinates) -> Option<&Entity> {
        if self.marked_tiles.contains(coords) {
            None
        } else {
            self.covered_tiles.get(coords)
        }
    }

    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        if self.marked_tiles.contains(coords) {
            self.unmark_tile(coords)?;
        }
        self.covered_tiles.remove(coords)
    }

    pub fn adjacent_covered_tiles(&self, coords: &Coordinates) -> impl Iterator<Item = &Entity> {
        coords.neighbors().filter_map(|c| self.get_covered_tile(&c))
    }

    fn unmark_tile(&mut self, coords: &Coordinates) -> Option<Coordinates> {
        let pos = match self.marked_tiles.iter().position(|a| a == coords) {
            Some(p) => p,
            None => {
                log::error!("Failed to unmark tile at {}", coords);
                return None;
            }
        };
        Some(self.marked_tiles.remove(pos))
    }

    pub fn try_toggle_mark(&mut self, coords: &Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coords)?;
        let mark = if self.marked_tiles.contains(coords) {
            self.unmark_tile(coords)?;
            false
        } else {
            self.marked_tiles.push(*coords);
            true
        };
        Some((entity, mark))
    }

    pub fn is_complete(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }

}
