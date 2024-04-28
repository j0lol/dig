use bevy::prelude::*;

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
                    scale: Vec2::new(1.0, 1.0).extend(1.0),
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
    commands.spawn(Camera2dBundle::default());
    for i in 0..32 {
        for j in 0..18 {
            commands.spawn(TileBundle::new(
                Vec2::new(i as f32 * 8., j as f32 * 8.),
                &asset_server,
            ));
        }
    }
}
