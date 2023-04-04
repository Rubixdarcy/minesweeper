use bevy::{prelude::*, window::PrimaryWindow, log, utils::HashMap, math::Vec3Swizzles};

use crate::{resources::{BoardOptions, tilemap::TileMap, TileSize, BoardPosition, tile::Tile, Board, BoardAssets}, bounds::Bounds2, components::{Coordinates, BombNeighbor, Bomb, Uncover}};

pub fn create_board(
    mut cmd: Commands,
    board_options: Option<Res<BoardOptions>>,
    board_assets: Res<BoardAssets>,
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

    let mut covered_tiles = HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
    let mut safe_start = None;

    let board_entity = cmd.spawn(Name::new("Board"))
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
                        color: board_assets.board_material.color,
                        custom_size: Some(board_size),
                        ..default()
                    },
                    texture: board_assets.board_material.texture.clone(),
                    transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    ..default()
                });

            spawn_tiles(
                board,
                &tile_map,
                tile_size,
                options.tile_padding,
                &board_assets,
                &mut covered_tiles,
                &mut safe_start,
            );
        })
        .id();
    
    if options.safe_start {
        if let Some(e) = safe_start {
            cmd.entity(e).insert(Uncover);
        }
    }
    
    cmd.insert_resource(Board {
        tile_map,
        bounds: Bounds2 {
            position: board_position.xy(),
            size: board_size,
        },
        tile_size,
        entity: board_entity,
        covered_tiles,
    });
}

fn spawn_tiles(
    parent: &mut ChildBuilder,
    tile_map: &TileMap,
    size: f32,
    padding: f32,
    board_assets: &BoardAssets,
    covered_tiles: &mut HashMap<Coordinates, Entity>,
    safe_start_entity: &mut Option<Entity>,
) {
    for (y, line) in tile_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let coords = Coordinates { x: x as u16, y: y as u16 };
            let mut cmd = parent.spawn_empty();
            cmd
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..default()
                    },
                    texture: board_assets.tile_material.texture.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coords)
                .with_children(|tile_entity| {
                    let entity = tile_entity.spawn(Name::new("Tile Cover"))
                        .insert(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: board_assets.covered_tile_material.color,
                                ..default()
                            },
                            texture: board_assets.covered_tile_material.texture.clone(),
                            transform: Transform::from_xyz(0., 0., 2.),
                            ..default()
                        })
                        .id();
                    covered_tiles.insert(coords, entity);
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

            match tile {
                Tile::Bomb => {
                    cmd.insert(Bomb)
                        .with_children(|p| {
                            p.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    color: board_assets.bomb_material.color,
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: board_assets.bomb_material.texture.clone(),
                                ..default()
                            });
                        });
                },
                Tile::BombNeighbor(n) => {
                    cmd.insert(BombNeighbor { count: *n })
                        .with_children(|p| {
                            p.spawn(bomb_count_text_bundle(*n, board_assets, size - padding));
                        });
                }
                Tile::Empty => (),
            }
        }
    }
}

pub fn despawn_board(mut cmd: Commands, board: Res<Board>) {
    log::info!("despawning board");
    cmd.entity(board.entity).despawn_recursive();
    cmd.remove_resource::<Board>();
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

fn bomb_count_text_bundle(count: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
    let text = count.to_string();
    let color = board_assets.bomb_counter_color(count);
    let style = TextStyle { font: board_assets.bomb_counter_font.clone(), font_size: size, color };
    let alignment = TextAlignment::Center;

    Text2dBundle {
        text: Text::from_section(text, style).with_alignment(alignment),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }
}
