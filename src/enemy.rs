use bevy::prelude::*;
use rand::{rand_core::le, random_range};
use crate::common::{Collider, AnimationTimer, AnimationIndices, Player};

/// Enum to distinguish between different types of enemies (i will use it in the future(i promise))
enum EnemyType {
    Orc,
    Wolf,
}

/// Timer resource used to control enemy spawn rate
#[derive(Resource)]
struct EnemyTimer(Timer);

/// Marker component for enemy entities, with a specific type
#[derive(Component)]
struct Enemy(EnemyType);

/// Plugin that handles spawning and ai(xd) for enemies
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyTimer(Timer::from_seconds(0.12, TimerMode::Repeating)))
            .add_systems(Update, (spawn_enemy, move_enemies));
    }
}

/// Spawns an enemy near the player when the 'O' key is held with 0.1 cd
fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_timer: ResMut<EnemyTimer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<&Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let spawn_range: f32 = 300.0;
    let random_x: f32;
    let random_y: f32;

    // Spawn only when 'O' is pressed and timer ticks
    if keyboard.pressed(KeyCode::KeyO) {
        if enemy_timer.0.tick(time.delta()).just_finished() {
            if let Ok(player_transform) = player_query.single() {
                let player_x = player_transform.translation.x;
                let player_y = player_transform.translation.y;

                // Generate a random spawn position around the player
                random_x = random_range(player_x - spawn_range..player_x + spawn_range);
                random_y = random_range(player_y - spawn_range..player_y + spawn_range);

                let texture = asset_server.load("Orc/Orc/Orc-Idle.png");
                let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                let animation_indices = AnimationIndices { first: 0, last: 5 };

                // Spawn the enemy entity with necessary components
                commands.spawn((
                    Sprite::from_atlas_image(
                        texture,
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: animation_indices.first,
                        },
                    ),
                    Enemy(EnemyType::Orc),
                    Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3 {
                        x: random_x,
                        y: random_y,
                        z: 0.0,
                    }),
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Collider { radius: 22.0 },
                ));
            }
        }
    }
}

/// Moves all enemies toward the player
fn move_enemies(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player.single() {
        for mut transform in enemies.iter_mut() {
            // Calculate direction to player and move enemy
            let dir = (player_transform.translation - transform.translation).normalize_or_zero();
            transform.translation += dir * time.delta_secs() * 100.0;

            // Flip sprite depending on movement direction
            if dir.x.abs() > 0.1 {
                transform.scale.x = dir.x.signum().abs() * 4.0 * dir.x.signum();
            }
        }
    }
}
