use bevy::prelude::*;
mod resources;
mod scene;
mod ui;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(scene::ScenePlugin)
    .add_plugins(ui::UiPlugin)
    .add_systems(Update, scene::inputs::camera_orbit_controls)
    .run();
}
