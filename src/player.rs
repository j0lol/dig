use bevy::{math::{vec2, I64Vec2}, prelude::*};

use crate::{camera::CameraMarker, grid::GridPos};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    pos: SubGridPos,
    player: Player,
    physics: Physics,
}

#[derive(Component, Default)]
pub struct SubGridPos(pub Vec2);

#[derive(Component, Default)]
pub struct Physics {
    pub velocity: Vec2,
}

impl PlayerBundle {
    fn new(location: I64Vec2, asset_server: &Res<AssetServer>) -> PlayerBundle {
        PlayerBundle {
            pos: SubGridPos(location.as_vec2()),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.extend(0).as_vec3(),
                    scale: Vec3::splat(1.0), // z component must be 1x scale in 2D
                    ..default()
                },
                texture: asset_server.load("creature.png"),
                ..default()
            },
            ..default()
        }
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (movement, center_camera_on_player).chain());
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

const FRICTION: f32 = 0.05;
const MAX_VELOCITY: f32 = 2.;

fn movement(mut player: Query<(&mut Transform, &mut Physics, &mut SubGridPos), With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    let Ok((mut transform, mut physics, mut pos)) = player.get_single_mut() else { return };

    let mut acceleration = if keys.pressed(KeyCode::KeyD) {
        Vec2::X * 0.1
    } else if keys.pressed(KeyCode::KeyA) {
        Vec2::NEG_X * 0.1
    } else {
        Vec2::ZERO
    };
    if keys.just_pressed(KeyCode::Space) {
        acceleration += Vec2::Y * 10.
    }

    physics.velocity = (physics.velocity + acceleration).clamp(vec2(-MAX_VELOCITY, -MAX_VELOCITY), vec2(MAX_VELOCITY, MAX_VELOCITY));

    if physics.velocity.x != 0.0 {
        let positive = physics.velocity.x.is_sign_positive();
        let friction = if positive { -FRICTION } else { FRICTION };
        physics.velocity.x += friction;

        // snapping force
        if physics.velocity.x.abs() < FRICTION && acceleration.x == 0.0 {
            physics.velocity.x = 0.;
        }
    }
    if physics.velocity.y != 0.0 {
        let positive = physics.velocity.y.is_sign_positive();
        let friction = if positive { -FRICTION } else { FRICTION };
        physics.velocity.y += friction;

        // snapping force
        if physics.velocity.y.abs() < FRICTION && acceleration.y == 0.0 {
            physics.velocity.y = 0.;
        }
    }

    dbg!(acceleration.x);
    dbg!(physics.velocity.x);

    pos.0 += physics.velocity;

    dbg!(pos.0);
    transform.translation = pos.0.round().extend(0.)
}
