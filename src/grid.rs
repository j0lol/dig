use bevy::{math::I64Vec2, prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

use crate::SpriteSheet;

const TILE_PX: f32 = 8.;

#[derive(Component, Default)]
pub struct Tile;

#[derive(Component)]
pub struct Cursor;

#[derive(Component, Default)]
pub struct GridPos {
    pub pos: IVec2,
}

#[derive(Bundle, Default)]
struct TileBundle {
    sprite_bundle: SpriteSheetBundle,
    grid_pos: GridPos,
    tile: Tile,
    // we should be able to add things here later to help w/ collision
}

impl TileBundle {
    fn new(location: IVec2, asset_server: &AssetServer, sprite_sheet: &SpriteSheet) -> TileBundle {

        TileBundle {
            grid_pos: GridPos { pos: location },
            sprite_bundle: SpriteSheetBundle {
                transform: Transform {
                    translation: (location * 8).extend(0).as_vec3(),
                    scale: Vec3::splat(1.0), // z component must be 1x scale in 2D
                    ..default()
                },
                texture: asset_server.load("tileset.png"),
                atlas: TextureAtlas { layout: sprite_sheet.0.clone(), index: 2 },
                ..default()
            },
            ..Default::default()
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_tiles, spawn_cursor))
            .add_systems(Update, (update_cursor, tile_interact).chain());
    }
}

fn spawn_tiles(mut commands: Commands, sprite_sheet: Res<SpriteSheet>, asset_server: Res<AssetServer>) {
    // let texture = asset_server.load("tileset.png").clone_weak();


    for i in -32..=32 {
        for j in -18..=0 {
            let location = IVec2::new(i, j);
            commands.spawn(
                TileBundle::new(location, &asset_server, &sprite_sheet)
            );
        }
    }
}

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>, sprite_sheet: Res<SpriteSheet>) {
    commands.spawn((Cursor, GridPos::default(), SpriteSheetBundle {
        transform: Transform {
            translation: Vec2::new(0.,0.).extend(0.),
            scale: Vec3::splat(1.0), // z component must be 1x scale in 2D
            ..default()
        },
        texture: asset_server.load("tileset.png"),
        atlas: TextureAtlas { layout: sprite_sheet.0.clone(), index: 0 },
        ..default()
    }));
}

fn update_cursor(
    mut tile_cursor: Query<(&mut Transform, &mut GridPos), With<Cursor>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<crate::camera::CameraMarker>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    let (mut tile_cursor_pos, mut pos) = tile_cursor.single_mut();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        tile_cursor_pos.translation = snap_to_grid(world_position).extend(10.);
        pos.pos = world_to_tile_coordinate(world_position);
    }
}

fn tile_interact(
    mut commands: Commands,
    tiles: Query<(Entity, &GridPos), With<Tile>>,
    cursor: Query<&GridPos, With<Cursor>>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    sprite_sheet: Res<SpriteSheet>
) {
    if buttons.all_pressed([MouseButton::Left, MouseButton::Right]) {
        return
    }
    let cursor_pos = cursor.single();

    if buttons.pressed(MouseButton::Left) {
        for (entity, pos) in tiles.iter() {
            if pos.pos == cursor_pos.pos {
                commands.entity(entity).despawn()
            }
        }
    }
    if buttons.pressed(MouseButton::Right) {
        let tile_at_cursor = tiles.iter().any(|(_, pos)| pos.pos == cursor_pos.pos);
        if !tile_at_cursor {
            commands.spawn(
                TileBundle::new(cursor_pos.pos, &asset_server, &sprite_sheet)
            );
        }
    }


}
fn world_to_tile_coordinate(world_pos: Vec2) -> IVec2 {
    let vec2 = world_pos / TILE_PX;
    vec2.round().as_ivec2()
}

fn tile_to_world_coordinate(tile_pos: IVec2) -> Vec2 {
    let ivec2 = tile_pos*(TILE_PX as i32);
    ivec2.as_vec2()
}
fn snap_to_grid(pos: Vec2) -> Vec2 {
    return tile_to_world_coordinate(world_to_tile_coordinate(pos))
}
