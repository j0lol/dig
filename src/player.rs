use bevy::{math::I64Vec2, prelude::*};

use crate::{camera::CameraMarker, grid::GridPos};

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    pos: SubGridPos,
    player: Player,
}

#[derive(Component)]
struct SubGridPos(I64Vec2);

impl PlayerBundle {
    fn new(location: I64Vec2, asset_server: &Res<AssetServer>) -> PlayerBundle {
        PlayerBundle {
            pos: SubGridPos(location),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.as_ivec2().extend(0).as_vec3(),
                    scale: Vec3::splat(1.0), // z component must be 1x scale in 2D
                    ..default()
                },
                texture: asset_server.load("creature.png"),
                ..default()
            },
            player: Player
        }
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (test_movement, center_camera_on_player).chain());
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(I64Vec2::new(0, 8), &asset_server));
}

fn center_camera_on_player(mut camera: Query<(&mut CameraMarker, &mut Transform), Without<Player>>, player: Query<&Transform, With<Player>>) {
    let Ok(player) = player.get_single() else { return };
    let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else { return };

    camera_transform.translation = player.translation;
}

fn test_movement(mut player: Query<&mut Transform, With<Player>>) {
    let Ok(mut transform) = player.get_single_mut() else { return };

    transform.translation.x += 0.1;
}
