use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};


#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct BoardAssets {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub bomb_counter_font: Handle<Font>,
    pub bomb_counter_colors: Vec<Color>,
    pub flag_material: SpriteMaterial,
    pub bomb_material: SpriteMaterial,
}

impl BoardAssets {
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }
    pub fn bomb_counter_color(&self, count: u8) -> Color {
        let count = count.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(count) {
            None => Color::WHITE,
            Some(c) => *c,
        }
    }
}
