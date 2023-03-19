use crate::{resources::tile::Tile, components::Coordinates};
use std::ops::{Deref, DerefMut};
use bevy::prelude::Resource;
use rand::{thread_rng, Rng};


/// Base tile map
#[derive(Resource, Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generates an empty map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();
        // Place bombs
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }
        // Place bomb neighbors
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
                if self.is_bomb_at(coords) {
                    continue;
                }
                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }
                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }

    fn is_bomb_at(&self, coords: Coordinates) -> bool {
        if coords.x >= self.width || coords.y >= self.height {
            return false;
        }
        self.map[coords.y as usize][coords.x as usize].is_bomb()
    }

    fn bomb_count_at(&self, coords: Coordinates) -> u8 {
        coords.neighbors()
            .filter(|&n| self.is_bomb_at(n))
            .count() as u8
    }


    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    // Getter for `width`
    pub fn width(&self) -> u16 {
        self.width
    }

    // Getter for `height`
    pub fn height(&self) -> u16 {
        self.height
    }

    // // Getter for `bomb_count`
    // pub fn bomb_count(&self) -> u16 {
    //     self.bomb_count
    // }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}