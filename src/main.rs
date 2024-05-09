mod grid;
mod player;
mod gravity;
mod camera;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use grid::{TilePlugin, TILE_PX_FLT};
use camera::CameraPlugin;
use player::PlayerPlugin;
use gravity::GravityPlugin;

fn main() {
    let plugins =
        (
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(TILE_PX_FLT),
            RapierDebugRenderPlugin::default(),
            CameraPlugin,
            TilePlugin,
            PlayerPlugin,
            GravityPlugin
        );
    App::new().add_plugins(plugins)
        .init_resource::<SpriteSheet>()
.run();
}


#[derive(Resource)]
struct SpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            Vec2::new(TILE_PX_FLT, TILE_PX_FLT), // The size of each image
            8, // The number of columns
            8, // The number of rows
            None, // Padding
            None // Offset
        );

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}
