use bevy::{reflect::Reflect, prelude::Component};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct BombNeighbor {
    /// Number of neighbor bombs
    pub count: u8,
}