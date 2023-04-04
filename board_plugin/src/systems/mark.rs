use bevy::{prelude::*, log};

use crate::{events::TileMarkEvent, resources::{Board, BoardAssets}};


pub fn mark_tiles(
    mut cmd: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    mut tile_mark_er: EventReader<TileMarkEvent>,
    query: Query<&Children>,
) {
    for evt in tile_mark_er.iter() {
        if let Some((entity, mark)) = board.try_toggle_mark(&evt.0) {
            if mark {
                cmd.entity(entity).with_children(|parent| {
                    parent
                        .spawn(Name::new("Flag"))
                        .insert(SpriteBundle {
                            texture: board_assets.flag_material.texture.clone(),
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(board.tile_size)),
                                color: board_assets.flag_material.color,
                                ..default()
                            },
                            ..default()
                        });
                });
            } else {
                let children = match query.get(entity) {
                    Ok(c) => c,
                    Err(e) => {
                        log::error!("Failed to retrieve flag entity components: {}", e);
                        continue;
                    }
                };
                for child in children.iter() {
                    cmd.entity(*child).despawn_recursive();
                }
            }
        }
    }

}
