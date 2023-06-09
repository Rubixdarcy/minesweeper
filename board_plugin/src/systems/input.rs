use bevy::{prelude::{Query, With, Res, EventReader, MouseButton, EventWriter}, window::{Window, PrimaryWindow}, input::{mouse::MouseButtonInput, ButtonState}, log};

use crate::{resources::Board, events::{TileTriggerEvent, TileMarkEvent}};


pub fn input_handling(
    windows: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ew: EventWriter<TileTriggerEvent>,
    mut tile_mark_ew: EventWriter<TileMarkEvent>,
) {
    let window = windows.single();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            tile_trigger_ew.send(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            log::info!("Trying to mark tile on {}", coordinates);
                            tile_mark_ew.send(TileMarkEvent(coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
