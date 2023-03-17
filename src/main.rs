use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_startup_system(sys_camera_setup);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();

}

fn sys_camera_setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}