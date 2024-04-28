use bevy::{math::I64Vec2, prelude::*, render::camera::ScalingMode};

const TILE_PX: f32 = 8.;

#[derive(Component)]
struct Tile;


#[derive(Component)]
struct GridPos {
    pos: I64Vec2,
}

#[derive(Bundle)]
struct TileBundle {
    sprite_bundle: SpriteBundle,
    grid_pos: GridPos,
    // we should be able to add things here later to help w/ collision
}

impl TileBundle {
    fn new(location: I64Vec2, asset_server: &Res<AssetServer>) -> TileBundle {
        TileBundle {
            grid_pos: GridPos { pos: location },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: (location.as_ivec2() * 8).extend(0).as_vec3(),
                    scale: Vec3::splat(1.0), // z component must be 1x scale in 2D
                    ..default()
                },
                texture: asset_server.load("tile.png"),
                ..default()
            },
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tiles);
    }
}

fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in -32..=32 {
        for j in -18..=0 {
            commands.spawn(TileBundle::new(
                I64Vec2::new(i, j),
                &asset_server,
            ));
        }
    }
}
