use bevy::{prelude::*, log};
use bevy_inspector_egui::quick::StateInspectorPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{BoardPlugin, BoardState};
use board_plugin::resources::BoardOptions;

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone, States, Reflect)]
pub enum AppState {
    #[default]
    InGame,
    Paused,
}

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(BoardPlugin)
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            safe_start: true,
            tile_padding: 2.,
            ..default()
        })
        .add_system(state_handler)
        .add_startup_system(sys_camera_setup)
        .configure_set(OnUpdate(BoardState::Active)
            .run_if(in_state(AppState::InGame)))

        .register_type::<AppState>()
    ;

    #[cfg(feature = "debug")]
    {
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(StateInspectorPlugin::<AppState>::new());
        app.add_plugin(StateInspectorPlugin::<BoardState>::new());
    }

    app.run();

}

fn state_handler(
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    board_state: Res<State<BoardState>>,
    mut next_board_state: ResMut<NextState<BoardState>>,
    keys: Res<Input<KeyCode>>,
) {

    enum StateControlKey { C, G, Esc }
    use StateControlKey::*;
    use AppState::*;
    use BoardState::*;

    let key_c = keys.just_pressed(KeyCode::C).then_some(C);
    let key_g = keys.just_pressed(KeyCode::G).then_some(G);
    let key_esc = keys.just_pressed(KeyCode::Escape).then_some(Esc);

    let Some(key) = key_c.or(key_g).or(key_esc) else { return; };

    let (message, app, board) = match (app_state.0, board_state.0, key) {
        (InGame, Inactive, G) => (Some("Starting game"), None, Some(Active)),
        (InGame, Active, G) => (Some("Restarting game"), None, Some(Active)),
        (InGame, Active, Esc) => (Some("Pausing game"), Some(Paused), None),
        (Paused, Active, Esc) => (Some("Unpausing game"), Some(InGame), None),
        (InGame, Active, C) => (Some("Stopping game"), None, Some(Inactive)),
        _ => (None, None, None),
    };

    if let Some(message) = message {
        log::info!(message);
    }
    if let Some(app) = app {
        next_app_state.set(app);
    }
    if let Some(board) = board {
        next_board_state.set(board);
    }
}

fn sys_camera_setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}