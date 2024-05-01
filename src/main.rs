mod grid;
mod player;
mod camera;
use bevy::prelude::*;
use grid::TilePlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    let plugins =
        (
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraPlugin,
            TilePlugin,
            PlayerPlugin
        );
    App::new().add_plugins(plugins).run();
}
