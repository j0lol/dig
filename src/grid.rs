use bevy::{math::I64Vec2, prelude::*, render::camera::ScalingMode};

const TILE_PX: f32 = 8.;
const PIXEL_SCALE: f32 = 1.;

#[derive(Component)]
struct Tile;


#[derive(Component)]
struct RenderPos {
    pos: I64Vec2,
}

#[derive(Bundle)]
struct TileBundle {
    sprite_bundle: SpriteBundle,
    // we should be able to add things here later to help w/ collision
}

impl TileBundle {
    fn new(location: Vec2, asset_server: &Res<AssetServer>) -> TileBundle {
        TileBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.extend(0.0),
                    scale: Vec2::splat(PIXEL_SCALE).extend(1.0),
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
    let factor = TILE_PX * PIXEL_SCALE;

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1./3.,
            // scaling_mode: ScalingMode::AutoMin { min_width: 320., min_height: 240. },
            far: 1000.,
            near: -1000.,
            ..default()
        },
        // projection: OrthographicProjection::default(),
        ..default()
    });
    // commands.spawn(Camera2dBundle::default());
    for i in 0..32 {
        for j in 0..18 {
            commands.spawn(TileBundle::new(
                Vec2::new(i as f32 * factor, j as f32 * factor),
                &asset_server,
            ));
        }
    }
}
