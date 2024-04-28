use bevy::prelude::*;

mod grid;
use grid::TilePlugin;
use camera::CameraPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), CameraPlugin, TilePlugin)).run();
}

mod camera {
    use super::*;

    pub struct CameraPlugin;

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, spawn);
        }
    }

    fn spawn(mut commands: Commands) {
        commands.spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1./3.,
                // scaling_mode: ScalingMode::AutoMin { min_width: 320., min_height: 240. },
                far: 1000.,
                near: -1000.,  // 2d boilerplate
                ..default()
            },
            ..default()
        });
    }
}
