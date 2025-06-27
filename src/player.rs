use bevy::platform::collections::HashMap;
use bevy::{prelude::*};
use crate::common::{Animation, AnimationIndices, AnimationSet, AnimationState, AttackEvent, Collider, HitReactionTimer, InvincibilityTimer, Player, Stats, Velocity};
use crate::enemy::Enemy;

pub struct PlayerPlugin;

/// Component used for managing animation timing and current frame
#[derive(Component)]
pub struct AnimationClock {
    pub frame: usize,    // Current animation frame index
    pub timer: Timer,    // Timer controlling the frame rate
}

/// Timer used to control the player's attack cooldown
#[derive(Component)]
pub struct PlayerAttackTimer {
    pub timer: Timer,
}

/// 4 movement directions for animation switching
#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents different animation states (idle, running, attacking)
#[derive(Clone, Debug, PartialEq)]
enum MovementState {
    Idle,
    Run,
    Attack01,
}

/// Combines the player's direction and animation state
#[derive(Component, Clone)]
struct PlayerAnimationState {
    direction: Direction,
    state: MovementState,
}


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (attack_player_system, control_player).chain());
    }
}

/// Spawns the player entity and initializes its components and animations
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load all textuares and layouts
    let idle_down = asset_server.load("Player/Sprites/IDLE/idle_down.png");
    let idle_up = asset_server.load("Player/Sprites/IDLE/idle_up.png");
    let idle_left = asset_server.load("Player/Sprites/IDLE/idle_left.png");
    let idle_right = asset_server.load("Player/Sprites/IDLE/idle_right.png");

    let run_down = asset_server.load("Player/Sprites/RUN/run_down.png");
    let run_up = asset_server.load("Player/Sprites/RUN/run_up.png");
    let run_left = asset_server.load("Player/Sprites/RUN/run_left.png");
    let run_right = asset_server.load("Player/Sprites/RUN/run_right.png");

    let attack_up = asset_server.load("Player/Sprites/ATTACK 1/attack1_up.png");
    let attack_down = asset_server.load("Player/Sprites/ATTACK 1/attack1_down.png");
    let attack_left = asset_server.load("Player/Sprites/ATTACK 1/attack1_left.png");
    let attack_right = asset_server.load("Player/Sprites/ATTACK 1/attack1_right.png");


    // Texture atlas layout for animations (8 frames horizontally)
    let layout_8fps = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(96), 8, 1, None, None));

    let indicies_8fps = AnimationIndices {
        first: 0,
        last: 7
    };

    let attack_indicies = AnimationIndices {
        first: 1,
        last: 4
    };

    // Declare animations map ;3
    let anim_map = HashMap::from([

        // Idle
        (AnimationState::IdleUp, (idle_up.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        (AnimationState::IdleDown, (idle_down.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        (AnimationState::IdleLeft, (idle_left.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        (AnimationState::IdleRight, (idle_right.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        
        // Run 
        (AnimationState::RunUp, (run_up.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        (AnimationState::RunDown, (run_down.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        (AnimationState::RunLeft, (run_left.clone(), layout_8fps.clone(), indicies_8fps.clone())),
        (AnimationState::RunRight, (run_right.clone(), layout_8fps.clone(), indicies_8fps.clone())),        

        // Attacks
        (AnimationState::AttackUp, (attack_up.clone(), layout_8fps.clone(), attack_indicies.clone())),
        (AnimationState::AttackDown, (attack_down.clone(), layout_8fps.clone(), attack_indicies.clone())),
        (AnimationState::AttackLeft, (attack_left.clone(), layout_8fps.clone(), attack_indicies.clone())),
        (AnimationState::AttackRight, (attack_right.clone(), layout_8fps.clone(), attack_indicies.clone())),
    ]);

    // declare animation set from map
    let set = AnimationSet { animations: anim_map };

    // component responsible for animation managment
    let anim = Animation {
        set,
        state: AnimationState::IdleDown,
        timer: Timer::from_seconds(0.125, TimerMode::Once),
        last_state: None
    };

    // Spawn the player with all necessary components
    commands.spawn((
        Sprite::from_atlas_image(
            idle_down.clone(),
            TextureAtlas {
                layout: layout_8fps.clone().clone(),
                index: 0,
            },
        ),
        Player,
        Transform::from_scale(Vec3::splat(2.0)),              // Set size
        Velocity(Vec3::ZERO),                                 // No initial velocity
        Collider { radius: 30.0 },                            // Collider size
        PlayerAttackTimer {
            timer: Timer::from_seconds(0.320, TimerMode::Once), 
        },
        Stats {
            hp: 100,
            max_hp: 100,
            attack: 30,
        },
        HitReactionTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        },
        InvincibilityTimer {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
        },
        anim
    ));
}


/// Controls player input, movement, attack triggering and camera follow
fn control_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(
        &mut Velocity,
        &mut Transform,
        &mut Animation,
        &mut PlayerAttackTimer,
    ), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok((mut velocity, mut player_transform, mut anim, mut attack_timer)) = player_query.single_mut() {
        velocity.0 = Vec3::ZERO;

        let mut new_state = anim.state;

        // Movement block if player is attacking
        if matches!(anim.state, AnimationState::AttackDown | AnimationState::AttackUp | AnimationState::AttackLeft | AnimationState::AttackRight)
            && !attack_timer.timer.finished()
        {
            // Player attacking
        } else {
            // player move
            if keyboard.pressed(KeyCode::KeyW) {
                velocity.0.y += 1.0;
                new_state = AnimationState::RunUp;
            }
            if keyboard.pressed(KeyCode::KeyS) {
                velocity.0.y -= 1.0;
                new_state = AnimationState::RunDown;
            }
            if keyboard.pressed(KeyCode::KeyA) {
                velocity.0.x -= 1.0;
                new_state = AnimationState::RunLeft;
            }
            if keyboard.pressed(KeyCode::KeyD) {
                velocity.0.x += 1.0;
                new_state = AnimationState::RunRight;
            }
        }

        println!("{}, {}", keyboard.pressed(KeyCode::Space), attack_timer.timer.finished());
        // player attack
        if keyboard.pressed(KeyCode::Space) && attack_timer.timer.finished() {
            attack_timer.timer.reset();
            println!("xd");
            new_state = match anim.state {
                AnimationState::RunUp | AnimationState::IdleUp => AnimationState::AttackUp,
                AnimationState::RunDown | AnimationState::IdleDown => AnimationState::AttackDown,
                AnimationState::RunLeft | AnimationState::IdleLeft => AnimationState::AttackLeft, 
                AnimationState::RunRight | AnimationState::IdleRight => AnimationState::AttackRight,
                _ => AnimationState::AttackDown,
            };

            velocity.0 = Vec3::ZERO; // Dont move
        } else if velocity.0.length_squared() == 0.0 {
            // If don't move - idle depending on the last direction
            new_state = match anim.state {
                AnimationState::RunUp | AnimationState::AttackUp => AnimationState::IdleUp,
                AnimationState::RunDown | AnimationState::AttackDown => AnimationState::IdleDown,
                AnimationState::RunLeft | AnimationState::AttackLeft => AnimationState::IdleLeft,
                AnimationState::RunRight | AnimationState::AttackRight => AnimationState::IdleRight,
                _ => anim.state,
            };
        }

        anim.state = new_state;

        // Move player
        player_transform.translation += velocity.0 * time.delta_secs() * 200.0;

        // Camera follow player
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}


fn attack_player_system(
    mut p_query: Query<(Entity, &Stats, &Transform, &Animation, &mut PlayerAttackTimer), With<Player>>, 
    e_query: Query<(Entity, &Transform), With<Enemy>>,
    mut attack_events: EventWriter<AttackEvent>,
    time: Res<Time>
) {
    if let Ok((player, p_stats, p_transform, p_anim_state, mut p_attack_timer)) = p_query.single_mut() {
        p_attack_timer.timer.tick(time.delta());

        if !p_attack_timer.timer.just_finished() {
            return;
        }

        let player_translation = p_transform.translation;
        let attack_range = 300.;

        let attack_offset = match p_anim_state.state {
            AnimationState::AttackUp => Vec3::Y * attack_range,
            AnimationState::AttackDown => -Vec3::Y * attack_range,
            AnimationState::AttackLeft => -Vec3::X * attack_range,
            AnimationState::AttackRight => Vec3::X * attack_range,
            _ => Vec3::ZERO
        };

        let attack_loc = player_translation + attack_offset;

        for (enemy, e_transform) in e_query.iter() {

            if e_transform.translation.distance(attack_loc) < attack_range {
                attack_events.write(AttackEvent { 
                    attacker: player, 
                    target: enemy, 
                    damage: p_stats.attack,
                });
            }
        }
    }
}
