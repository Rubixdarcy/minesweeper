use bevy::prelude::*;
use bevy::log;
use bevy::window::PrimaryWindow;
use resources::BoardOptions;

use crate::components::Coordinates;
use crate::resources::BoardPosition;
use crate::resources::TileSize;
use crate::resources::tilemap::TileMap;

mod components;
pub mod resources;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::sys_create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {

    pub fn sys_create_board(
        mut cmd: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = windows.single();
        let options = board_options.map(|o| o.to_owned()).unwrap_or_default();

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(s) => s,
            TileSize::Adaptive { min, max } =>
                Self::adaptative_tile_size(window, (min, max), (options.map_size.0, options.map_size.1))
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        cmd.spawn(Name::new("Board"))
            .insert(SpatialBundle {
                transform: Transform::from_translation(board_position),
                visibility: Visibility::Visible,
                ..default()
            })
            .with_children(|board| {
                board
                    .spawn(Name::new("Background"))
                    .insert(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    });
                
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, tile) in line.iter().enumerate() {
                        board
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some((tile_size, tile_size).into()),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.,
                                ),
                                ..default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            .insert(Coordinates { x: x as u16, y: y as u16 });
                    }
                }
            });

    }

    fn adaptative_tile_size(
        window: &Window,
        (min, max): (f32, f32), // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_heigth = window.height() / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }
}