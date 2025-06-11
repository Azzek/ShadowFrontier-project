use bevy::{prelude::*, state::commands};
use crate::common::{Velocity, Player, Collider};

/// Plugin responsible for spawning and controlling the player
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player_and_camera);
    }
}

/// Spawns the player entity with a sprite, velocity, and collider
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("cb89ba6d036bf32.png");

    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
            ..default()
        },
        Player,
        Transform::default(),
        Velocity(Vec3::ZERO),
        Collider { radius: 40.0 },
    ));
}

/// Handles player movement and camera following logic
fn move_player_and_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok((mut velocity, mut player_transform)) = player_query.single_mut() {
        velocity.0 = Vec3::ZERO;

        // Read input and set direction
        if keyboard.pressed(KeyCode::KeyW) {
            velocity.0.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            velocity.0.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            velocity.0.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity.0.x += 1.0;
        }

        // Move the player based on velocity and delta time
        player_transform.translation += velocity.0 * time.delta_secs() * 200.0;

        // Move the camera to follow the player
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
