use bevy::prelude::*;

const BG_COLOR: Color = Color::rgb(62./255., 4./255., 45./255.);
pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraMarker;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1./2.,
            // scaling_mode: ScalingMode::AutoMin { min_width: 320., min_height: 240. },
            far: 1000.,
            near: -1000.,  // 2d boilerplate
            ..default()
        },
        ..default()
    }, CameraMarker));
}
