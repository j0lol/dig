use bevy::prelude::*;
use crate::{grid::TILE_PX_FLT, player::*};
pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}

fn apply_gravity(mut player: Query<(&mut Transform, &mut Physics, &mut SubGridPos), With<Player>>) {
    let Ok((mut transform, mut physics, mut pos)) = player.get_single_mut() else { return };

    if pos.0.y > TILE_PX_FLT {
        physics.velocity.y -= 0.1;
    } else {
        pos.0.y = TILE_PX_FLT
    }
}
