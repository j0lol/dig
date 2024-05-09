use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use crate::SpriteSheet;

pub const TILE_PX: i32 = 16;
pub const TILE_PX_FLT: f32 = TILE_PX as f32;

pub enum TileDirection {
    CENTER = 9,
    TOP = 1
}

#[derive(Component, Default)]
pub struct Tile;

#[derive(Component)]
pub struct Cursor;

#[derive(Component, Default, Clone, Copy)]
pub struct GridPos(IVec2);

#[derive(Component, Default, Clone, Copy)]
pub struct WorldPos(Vec2);

impl GridPos {
    pub fn as_world(&self) -> WorldPos {
        let ivec2 = self.0*TILE_PX;
        WorldPos(ivec2.as_vec2())
    }
}
impl WorldPos {
    pub fn as_grid(&self) -> GridPos {
        let vec2 = self.0 / TILE_PX_FLT;
        GridPos(vec2.round().as_ivec2())
    }
    pub fn grid_snap(&self) -> Self {
        return self.as_grid().as_world()
    }
}

#[derive(Bundle, Default)]
struct TileBundle {
    collider: Collider,
    sprite_bundle: SpriteSheetBundle,
    grid_pos: GridPos,
    tile: Tile,
    // we should be able to add things here later to help w/ collision
}

impl TileBundle {
    fn new(pos: GridPos, asset_server: &AssetServer, sprite_sheet: &SpriteSheet, direction: TileDirection) -> TileBundle {

        TileBundle {
            collider: Collider::cuboid(TILE_PX_FLT/2., TILE_PX_FLT/2.),
            grid_pos: pos,
            sprite_bundle: SpriteSheetBundle {
                transform: Transform {
                    translation: pos.as_world().0.extend(0.),
                    scale: Vec3::splat(1.0), // z component must be 1x scale in 2D
                    ..default()
                },
                texture: asset_server.load("tileset.png"),
                atlas: TextureAtlas { layout: sprite_sheet.0.clone(), index: direction as usize },
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
    for i in -64..=64 {
        let location = GridPos(IVec2::new(i, 0));
        commands.spawn(
            TileBundle::new(location, &asset_server, &sprite_sheet, TileDirection::TOP)
        );
    }
    for i in -64..=64 {
        for j in -64..=-1 {
            let location = GridPos(IVec2::new(i, j));
            commands.spawn(
                TileBundle::new(location, &asset_server, &sprite_sheet, TileDirection::CENTER)
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
        atlas: TextureAtlas { layout: sprite_sheet.0.clone(), index: 4 },
        ..default()
    }));
}

fn update_cursor(
    mut tile_cursor: Query<(&mut Transform, &mut GridPos), With<Cursor>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
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
        let world_position = WorldPos(world_position);
        tile_cursor_pos.translation = world_position.grid_snap().0.extend(10.);
        pos.0 = world_position.as_grid().0;
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
            if pos.0 == cursor_pos.0 {
                commands.entity(entity).despawn()
            }
        }
    }
    if buttons.pressed(MouseButton::Right) {
        let tile_at_cursor = tiles.iter().any(|(_, pos)| pos.0 == cursor_pos.0);
        if !tile_at_cursor {
            commands.spawn(
                TileBundle::new(*cursor_pos, &asset_server, &sprite_sheet, TileDirection::CENTER)
            );
        }
    }
}
