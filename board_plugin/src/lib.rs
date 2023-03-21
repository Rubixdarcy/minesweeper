use bevy::prelude::*;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::window::PrimaryWindow;
use resources::BoardOptions;
use resources::tile::Tile;

use crate::bounds::Bounds2;
use crate::components::Bomb;
use crate::components::BombNeighbor;
use crate::components::Coordinates;
use crate::components::Uncover;
use crate::resources::Board;
use crate::resources::BoardPosition;
use crate::resources::TileSize;
use crate::resources::tilemap::TileMap;

mod bounds;
mod components;
pub mod resources;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {

        app.register_type::<Bomb>();
        app.register_type::<BombNeighbor>();
        app.register_type::<Uncover>();

        app.add_startup_system(Self::sys_create_board);
        app.add_system(systems::input::input_handling);

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {

    pub fn sys_create_board(
        mut cmd: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {

        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");

        let window = windows.single();
        let options = board_options.map(|o| o.to_owned()).unwrap_or_default();

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(s) => s,
            TileSize::Adaptive { min, max } =>
                adaptative_tile_size(window, (min, max), (options.map_size.0, options.map_size.1))
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
                spawn_tiles(
                    board,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                );
            });
        
        cmd.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
        });
    }

}

fn spawn_tiles(
    parent: &mut ChildBuilder,
    tile_map: &TileMap,
    size: f32,
    padding: f32,
    _color: Color,
    bomb_image: Handle<Image>,
    font: Handle<Font>,
) {
    for (y, line) in tile_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let coords = Coordinates { x: x as u16, y: y as u16 };
            let mut cmd = parent.spawn_empty();
            cmd
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some((size, size).into()),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coords);

            match tile {
                Tile::Bomb => {
                    cmd.insert(Bomb)
                        .with_children(|p| {
                            p.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..default()
                            });
                        });
                },
                Tile::BombNeighbor(n) => {
                    cmd.insert(BombNeighbor { count: *n })
                        .with_children(|p| {
                            p.spawn(bomb_count_text_bundle(*n, font.clone(), size - padding));
                        });
                }
                Tile::Empty => (),
            }
        }
    }
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

fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
    let text = count.to_string();
    let color = match count {
        1 => Color::WHITE,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::ORANGE,
        _ => Color::PURPLE,
    };
    let style = TextStyle { font, font_size: size, color };
    let alignment = TextAlignment::Center;

    Text2dBundle {
        text: Text::from_section(text, style).with_alignment(alignment),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }
}