use bevy::{window::Window, prelude::{Vec2, Resource, Entity}, utils::HashMap};

use crate::{bounds::Bounds2, components::Coordinates};

use super::tilemap::TileMap;


#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
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
        self.covered_tiles.get(coords)
    }

    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coords)
    }

    pub fn adjacent_covered_tiles(&self, coords: &Coordinates) -> impl Iterator<Item = &Entity> {
        coords.neighbors().filter_map(|c| self.get_covered_tile(&c))
    }

}