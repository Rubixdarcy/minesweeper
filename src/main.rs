use bevy::{prelude::*, log};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, States)]
pub enum AppState {
    InGame,
    #[default]
    Out,
}

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(BoardPlugin { running_state: AppState::InGame })
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            safe_start: true,
            tile_padding: 2.,
            ..default()
        })
        .add_system(state_handler)
        .add_startup_system(sys_camera_setup);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();

}

fn state_handler(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.0 == AppState::InGame {
            log::info!("clearing game");
            next_state.set(AppState::Out);
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.0 == AppState::Out {
            log::info!("loading game");
            next_state.set(AppState::InGame);
        }
    }
}

fn sys_camera_setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}