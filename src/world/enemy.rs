use bevy::{ platform::collections::HashMap, prelude::*, window::PrimaryWindow };
use crate::{core::common::{
    Animation, AnimationIndices, AnimationSet, AnimationState, AttackEvent, Collider, HitReactionTimer, Player, Stats, Target
}, world::minnions::minnion::Minnion};

use std::time::Duration;

/// Enum to distinguish between different types of enemies 
enum EnemyType {
    Orc,
    Wolf,
}

/// Component holding a timer for enemy attack cooldown
#[derive(Component)]
struct EnemyAttackTimer {
    timer: Timer,
}

/// Global timer resource used to limit how often enemies can be spawned
#[derive(Resource)]
struct EnemyTimer(Timer);

/// Marker component for enemy entities, includes their type
#[derive(Component)]
pub struct Enemy(EnemyType);

/// Plugin responsible for handling enemy logic: spawning, movement, and attacking
pub struct EnemyPlugin;

// Plugin for enemies and its behavior
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyTimer(Timer::from_seconds(0.12, TimerMode::Repeating))) // Set enemy spawn rate
            .add_systems(Update, (
                spawn_enemy,
                find_enemy_target, 
                move_enemies_tow_target, 
                enemy_attack, 
                hit_reaction, 
                drop_target, 
                change_animation_state
            )); // Register systems
    }
}

/// Spawns an Orc enemy when the 'O' key is pressed, with a 0.12s cooldown
fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_timer: ResMut<EnemyTimer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {

    if !keyboard.pressed(KeyCode::KeyO) || !enemy_timer.0.tick(time.delta()).finished() {
        return;
    }

    // Pozycja kursora w world-space
    let cursor_pos = q_window
        .single()
        .ok()
        .and_then(|w| w.cursor_position())
        .and_then(|pos| {
            let (cam, cam_tf) = q_camera.single().ok()?;
            cam.viewport_to_world(cam_tf, pos).map(|r| r.origin.truncate()).ok()
        });

    let cursor_pos = if let Some(pos) = cursor_pos { pos } else { return };

    
    // Load textures for different animations
    let walk_tex = asset_server.load("Orc/Orc/Orc-Walk.png");
    let atk_tex  = asset_server.load("Orc/Orc/Orc-Attack01.png");
    let idle_tex = asset_server.load("Orc/Orc/Orc-Idle.png");
    let hurt_tex = asset_server.load("Orc/Orc/Orc-Hurt.png");
    
    let hurt_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 4, 1, None, None));
  
    let anim_layout   = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None));
    // let attack_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None));
    // let idle_layout   = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None));

    let anim_map = HashMap::from([
        (AnimationState::Idle,   (idle_tex.clone(), anim_layout.clone(), AnimationIndices { first: 0, last: 5 })),
        (AnimationState::Walk,   (walk_tex.clone(), anim_layout.clone(), AnimationIndices { first: 0, last: 5 })),
        (AnimationState::Attack01, (atk_tex.clone(),  anim_layout.clone(), AnimationIndices { first: 0, last: 5 })),
        (AnimationState::Hurt,   (hurt_tex.clone(), hurt_layout.clone(), AnimationIndices { first: 0, last: 1 })),
    ]);

    let animation_set = AnimationSet{ animations: anim_map };
    
    let mut hit_timer = HitReactionTimer {
        timer: Timer::from_seconds(0.2, TimerMode::Once),
    };

    hit_timer.timer.tick(Duration::from_secs_f32(0.2));

    let animation = Animation {
        set: animation_set,
        state: AnimationState::Idle,
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        last_state: None
    };
    // Spawn the enemy entity with all required components
    commands.spawn((
        Sprite::from_atlas_image(
            idle_tex,
            TextureAtlas {
                layout: anim_layout,
                index: 0,
            },
        ),
        Enemy(EnemyType::Orc),
        Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3 {
            x: cursor_pos.x,
            y: cursor_pos.y,
            z: 0.0,
        }),
        animation,
        Collider { radius: 22.0 },
        EnemyAttackTimer {
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
        },
        Stats { 
            hp: 100,
            max_hp: 100,
            attack: 30,
        },
        hit_timer
    ));
    
    
}


/// System that moves enemies towards their target (if any),
/// as long as they are not currently under attack.
fn move_enemies_tow_target(
    mut enemies: Query<(&mut Transform, &EnemyAttackTimer, Option<&Target>), With<Enemy>>,
    targets: Query<&Transform, Without<Enemy>>,
    time: Res<Time>,
) {
    for (mut enemy_tf, attack_timer, maybe_target) in enemies.iter_mut() {
        // If the enemy has a target assigned
        if let Some(target) = maybe_target {
            if let Ok(target_tf) = targets.get(target.target) {
                // It only moves if it doesn't attack
                if !attack_timer.timer.finished() {
                    let direction = (target_tf.translation - enemy_tf.translation).normalize_or_zero();
                    
                    // Movement towards the target
                    enemy_tf.translation += direction * time.delta_secs() * 100.0;

                    // Rotate the sprite towards the target
                    if direction.x.abs() > 0.1 {
                        enemy_tf.scale.x = direction.x.signum() * 4.0;
                    }
                }
            }
        }
    }
}


// Finds target for enemy 
fn find_enemy_target(
    enemies: Query<(Entity, &Transform), (With<Enemy>, (Without<Player>, Without<Minnion>, Without<Target>))>,
    targets: Query<(Entity, &Transform), (Or<(With<Player>, With<Minnion>)>, Without<Enemy>)>,
    mut commands: Commands,
) {
    for (enemy, enemy_tf) in enemies.iter() {
        let mut closest_target: Option<(Entity, f32)> = None;

        for (target, target_tf) in targets.iter() {
            let dist = enemy_tf.translation.distance(target_tf.translation);
            if dist < 500.0 {
                match closest_target {
                    Some((_, closest_dist)) if dist < closest_dist => {
                        closest_target = Some((target, dist));
                    }
                    None => {
                        closest_target = Some((target, dist));
                    }
                    _ => {}
                }
            }
        }

        if let Some((ent, _)) = closest_target {
            commands.entity(enemy).insert(Target { target: ent });
        } else {
            commands.entity(enemy).remove::<Target>();
        }
    }
}


// Drops target when distanse id greater than 550
fn drop_target(
    mut query: Query<(Entity, &mut Target, &Transform), With<Enemy>>,
    targets: Query<&Transform>,
    mut commands: Commands
) {
    for (minnion_entity, target, tf) in query.iter_mut() {

        match targets.get(target.target) {
            Ok(target_tf) => {
                if tf.translation.distance(target_tf.translation) > 550.0 {
                    commands.entity(minnion_entity).remove::<Target>();
                }
            }
            Err(_) => {
                commands.entity(minnion_entity).remove::<Target>();
            }
        }
    }
}


/// Handles enemy attack logic when in range of the player
fn enemy_attack(
    mut enemy_q: Query<(Entity, &Transform, &mut EnemyAttackTimer, &HitReactionTimer, Option<&Target>), With<Enemy>>,
    targets_q: Query<&Transform, Without<Enemy>>,
    time: Res<Time>,
    mut attack_events: EventWriter<AttackEvent>,
) {

    for (enemy_ent, enemy_tf, mut cooldown, hit_timer, maybe_target) in enemy_q.iter_mut() {

        if let Some(target) = maybe_target {
            cooldown.timer.tick(time.delta());
            if let Ok(target_tf) = targets_q.get(target.target) {
                let dist = enemy_tf.translation.distance(target_tf.translation);

                if hit_timer.timer.finished() {
                    if dist < 110.  {

                        if cooldown.timer.just_finished() {
                            attack_events.write(AttackEvent {
                                attacker: enemy_ent,
                                target: target.target,
                                damage: 30,
                            });
                            cooldown.timer.reset();
                        }
                    }
                }
            }
        }
        
    }
    
}

fn change_animation_state(
    q: Query<(&HitReactionTimer, &EnemyAttackTimer, &mut Animation, &Transform, Option<&Target>), With<Enemy>>,
    targets_q: Query<&Transform, (Or<(With<Player>, With<Minnion>)>, Without<Enemy>)>
) {

    for (hit_timer, att_timer,mut anim, tf,  maybe_target) in q {
        if !hit_timer.timer.finished() {
            anim.state = AnimationState::Hurt;
            continue;
        }

        if let Some(target) = maybe_target {
            if let Ok(target_tf) = targets_q.get(target.target) {

                if tf.translation.distance(target_tf.translation) < 110. && !att_timer.timer.finished(){
                    anim.state = AnimationState::Attack01;
                    continue;
                } else {
                    anim.state = AnimationState::Walk;
                    continue;
                }
            }
        }

        anim.state = AnimationState::Idle;
    }
}


fn hit_reaction(
    mut commands: Commands,
    mut query: Query<(Entity, &Stats, &mut HitReactionTimer), With<Enemy>>,
    time: Res<Time>,
) {
    for (enemy, stats, mut reaction_timer) in query.iter_mut() {
        reaction_timer.timer.tick(time.delta());

        if stats.hp <= 0 {
            commands.entity(enemy).despawn();
            continue;
        }
    }
}