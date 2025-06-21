use bevy::{prelude::*};
use crate::common::{AttackEvent, Collider, HitReactionTimer, InvincibilityTimer, Player, Stats, Velocity};
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

/// Holds all directional animations for each state (run, idle, attack)
#[derive(Component, Default)]
pub struct PlayerAnimationSet {
    run_down: Handle<Image>,
    run_up: Handle<Image>,
    run_left: Handle<Image>,
    run_right: Handle<Image>,

    idle_down: Handle<Image>,
    idle_up: Handle<Image>,
    idle_left: Handle<Image>,
    idle_right: Handle<Image>,

    attack_up: Handle<Image>,
    attack_down: Handle<Image>,
    attack_left: Handle<Image>,
    attack_right: Handle<Image>,

    layout: Handle<TextureAtlasLayout>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (attack_player_system, control_player, animate_player, ).chain());
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
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(96), 8, 1, None, None));

    // Spawn the player with all necessary components
    commands.spawn((
        Sprite::from_atlas_image(
            idle_down.clone(),
            TextureAtlas {
                layout: layout.clone(),
                index: 0,
            },
        ),
        Player,
        Transform::from_scale(Vec3::splat(2.0)),              // Set size
        Velocity(Vec3::ZERO),                                 // No initial velocity
        Collider { radius: 40.0 },                            // Collider size
        AnimationClock {
            frame: 0,
            timer: Timer::from_seconds(0.1667, TimerMode::Repeating),  // 10 FPS
        },
        PlayerAnimationState {
            direction: Direction::Down,
            state: MovementState::Idle,
        },
        PlayerAnimationSet {
            run_down,
            run_up,
            run_left,
            run_right,
            idle_down,
            idle_up,
            idle_left,
            idle_right,
            attack_up,
            attack_down,
            attack_left,
            attack_right,
            layout,
        },
        PlayerAttackTimer {
            timer: Timer::from_seconds(0.6, TimerMode::Once), // Half-second cooldown
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
    ));
}

/// Animates the player sprite based on their current state and direction
fn animate_player(
    time: Res<Time>,
    mut query: Query<(
        &PlayerAnimationSet,
        &PlayerAnimationState,
        &mut Sprite,
        &mut AnimationClock,
    ), With<Player>>,
) {
    for (animation_set, anim_state, mut sprite, mut clock) in query.iter_mut() {
        clock.timer.tick(time.delta());

        if clock.timer.finished() {
            clock.frame = (clock.frame + 1) % 6; // Loop through 6 frames

            // Select the appropriate animation texture
            let image_handle = match (&anim_state.direction, &anim_state.state) {
                (Direction::Down, MovementState::Run) => &animation_set.run_down,
                (Direction::Up, MovementState::Run) => &animation_set.run_up,
                (Direction::Left, MovementState::Run) => &animation_set.run_left,
                (Direction::Right, MovementState::Run) => &animation_set.run_right,

                (Direction::Down, MovementState::Idle) => &animation_set.idle_down,
                (Direction::Up, MovementState::Idle) => &animation_set.idle_up,
                (Direction::Left, MovementState::Idle) => &animation_set.idle_left,
                (Direction::Right, MovementState::Idle) => &animation_set.idle_right,

                (Direction::Up, MovementState::Attack01) => &animation_set.attack_up,
                (Direction::Down, MovementState::Attack01) => &animation_set.attack_down,
                (Direction::Left, MovementState::Attack01) => &animation_set.attack_left,
                (Direction::Right, MovementState::Attack01) => &animation_set.attack_right,
            };

            // Update sprite with new frame and texture
            *sprite = Sprite::from_atlas_image(
                image_handle.clone(),
                TextureAtlas {
                    layout: animation_set.layout.clone(),
                    index: clock.frame,
                },
            );
        }
    }
}

/// Controls player input, movement, attack triggering and camera follow
fn control_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(
        &mut Velocity,
        &mut Transform,
        &mut PlayerAnimationState,
        &mut PlayerAttackTimer,
    ), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok((mut velocity, mut player_transform, mut anim_state, mut attack_timer)) = player_query.single_mut() {
        velocity.0 = Vec3::ZERO;

        let mut new_state = anim_state.state.clone();
        let mut new_direction = anim_state.direction.clone();

        // Prevent movement during attack animation
        if matches!(anim_state.state, MovementState::Attack01) && !attack_timer.timer.finished() {
            // Player is attacking — block movement
        } else {
            // Handle movement input
            if keyboard.pressed(KeyCode::KeyW) {
                velocity.0.y += 1.0;
                new_direction = Direction::Up;
            }
            if keyboard.pressed(KeyCode::KeyS) {
                velocity.0.y -= 1.0;
                new_direction = Direction::Down;
            }
            if keyboard.pressed(KeyCode::KeyA) {
                velocity.0.x -= 1.0;
                new_direction = Direction::Left;
            }
            if keyboard.pressed(KeyCode::KeyD) {
                velocity.0.x += 1.0;
                new_direction = Direction::Right;
            }

            // Handle attack input
            if keyboard.pressed(KeyCode::Space) && attack_timer.timer.finished() {
                attack_timer.timer.reset();
                new_state = MovementState::Attack01;
                velocity.0 = Vec3::ZERO; // Player doesn't move during attack
            } else if velocity.0.length_squared() > 0.0 {
                new_state = MovementState::Run;
            } else {
                new_state = MovementState::Idle;
            }
        }

        // Apply new animation state
        *anim_state = PlayerAnimationState {
            direction: new_direction,
            state: new_state,
        };

        // Move player based on velocity and delta time
        player_transform.translation += velocity.0 * time.delta_secs() * 200.0;

        // Make the camera follow the player
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn attack_player_system(
    mut p_query: Query<(Entity, &Stats, &Transform, &PlayerAnimationState, &mut PlayerAttackTimer), With<Player>>, 
    e_query: Query<(Entity, &Transform), With<Enemy>>,
    mut attack_events: EventWriter<AttackEvent>,
    time: Res<Time>
) {
    if let Ok((player, p_stats, p_transform, p_anim_state, mut p_attack_timer)) = p_query.single_mut() {
        p_attack_timer.timer.tick(time.delta());

        if (p_anim_state.state != MovementState::Attack01) || !p_attack_timer.timer.just_finished() {
            return;
        }

        let player_translation = p_transform.translation;
        let attack_range = 300.;

        let attack_offset = match p_anim_state.direction {
            Direction::Up => Vec3::Y * attack_range,
            Direction::Down => -Vec3::Y * attack_range,
            Direction::Left => -Vec3::X * attack_range,
            Direction::Right => Vec3::X * attack_range,
        };

        let attack_loc = player_translation + attack_offset;

        for (enemy, e_transform) in e_query.iter() {

            if e_transform.translation.distance(attack_loc) < attack_range {
                println!("{}", e_transform.translation.distance(attack_loc));
                attack_events.write(AttackEvent { 
                    attacker: player, 
                    target: enemy, 
                    damage: p_stats.attack,
                });
            }
        }
    }
}
