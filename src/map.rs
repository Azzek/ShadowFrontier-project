use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map);
    }
}

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let map_handle = asset_server.load("maps/untitled.tmx");
    commands.spawn((
        TiledMapHandle(map_handle),
        TilemapAnchor::Center,
        Transform {
            translation: Vec3::new(0., 0., -1000.0), // <- tutaj z ujemne
            scale: Vec3::splat(2.5),
            ..default()
        },
    ));
}
