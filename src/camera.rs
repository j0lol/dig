use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraMarker;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1./3.,
            // scaling_mode: ScalingMode::AutoMin { min_width: 320., min_height: 240. },
            far: 1000.,
            near: -1000.,  // 2d boilerplate
            ..default()
        },
        ..default()
    }, CameraMarker));
}
