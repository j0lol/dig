use bevy::prelude::*;

use grid::TilePlugin;
mod grid;

fn main() {
    App::new().add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), TilePlugin)).run();
}
