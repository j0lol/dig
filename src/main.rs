mod grid;
mod player;
mod gravity;
mod camera;
use bevy::prelude::*;
use grid::TilePlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use gravity::GravityPlugin;

fn main() {
    let plugins =
        (
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraPlugin,
            TilePlugin,
            PlayerPlugin,
            GravityPlugin
        );
    App::new().add_plugins(plugins).run();
}
