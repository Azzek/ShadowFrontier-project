use bevy::prelude::*;
use crate::{common::{Collider, Player, Velocity}};
/// Plugin responsible for spawning and controlling the player
pub struct PlayerPlugin;

#[derive(Component)]
pub struct AnimationClock {
    pub frame: usize,
    pub timer: Timer,
}

#[derive(Component)]
struct PlayerAttackTimer {
    timer: Timer
}


#[derive(Component, Default)]
struct  PlayerAnimationSet {
    run_down: Handle<Image>,
    run_up: Handle<Image>,
    run_left: Handle<Image>,
    run_right: Handle<Image>,

    idle_down: Handle<Image>,
    idle_up: Handle<Image>,
    idle_left: Handle<Image>,
    idle_right: Handle<Image>,

    layout: Handle<TextureAtlasLayout>
}

#[derive(Component, Debug, Clone)]
enum PlayerAnimationState {
    RunDown,
    RunUp,
    RunLeft,
    RunRight,
    IdleDown,
    IdleUp,
    IdleLeft,
    IdleRight,
    
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, control_player)
            .add_systems(Update, animate_player);
    }
}

/// Spawns the player entity with a sprite, velocity, and collider
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let idle_down: Handle<Image> = asset_server.load("Player/Sprites/IDLE/idle_down.png");
    let idle_up: Handle<Image> = asset_server.load("Player/Sprites/IDLE/idle_up.png");
    let idle_left: Handle<Image> = asset_server.load("Player/Sprites/IDLE/idle_left.png");
    let idle_right: Handle<Image> = asset_server.load("Player/Sprites/IDLE/idle_right.png");
    
    let run_down: Handle<Image> = asset_server.load("Player/Sprites/RUN/run_down.png");
    let run_up: Handle<Image> = asset_server.load("Player/Sprites/RUN/run_up.png");
    let run_left: Handle<Image> = asset_server.load("Player/Sprites/RUN/run_left.png");
    let run_right: Handle<Image> = asset_server.load("Player/Sprites/RUN/run_right.png");

    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(96), 8, 1, None, None));
    
    
    commands.spawn((
        Sprite::from_atlas_image(
            idle_down.clone(),
            TextureAtlas { 
                layout: layout.clone(),
                index: 0
            }),
        Player,
        Transform::from_scale(Vec3::splat(2.0)),
        Velocity(Vec3::ZERO),
        Collider { radius: 40.0 },
        
        AnimationClock {
            frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        PlayerAnimationState::RunDown,
        PlayerAnimationSet { 
        run_down: run_down,
        run_up: run_up,
        run_left: run_left,
        run_right: run_right,
        
        idle_down: idle_down,
        idle_up: idle_up,
        idle_left: idle_left,
        idle_right: idle_right,

        layout: layout
        },
        PlayerAttackTimer{timer:Timer::from_seconds(0.2, TimerMode::Once)}
    ));
}


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
            clock.frame = (clock.frame + 1) % 6; 

            let image_handle = match anim_state {
                PlayerAnimationState::RunDown => &animation_set.run_down,
                PlayerAnimationState::RunUp => &animation_set.run_up,
                PlayerAnimationState::RunLeft => &animation_set.run_left,
                PlayerAnimationState::RunRight => &animation_set.run_right,
                PlayerAnimationState::IdleDown => &animation_set.idle_down,
                PlayerAnimationState::IdleUp => &animation_set.idle_up,
                PlayerAnimationState::IdleLeft => &animation_set.idle_left,
                PlayerAnimationState::IdleRight => &animation_set.idle_right,
            };

            *sprite = Sprite::from_atlas_image(
                image_handle.clone(),
                TextureAtlas {
                    layout: animation_set.layout.clone(),
                    index: clock.frame,
                },
            );
            println!("{:?}", *anim_state)
        }
    }
}


/// Handles player movement and camera following logic
fn control_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse:Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut PlayerAnimationState), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok((mut velocity, mut player_transform, mut anim_state)) = player_query.single_mut() {
        velocity.0 = Vec3::ZERO;

        // Read input and set direction
        let mut moving = false;

        if keyboard.pressed(KeyCode::KeyW) {
            velocity.0.y += 1.0;
            *anim_state = PlayerAnimationState::RunUp;
            moving = true;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            velocity.0.y -= 1.0;
            *anim_state = PlayerAnimationState::RunDown;
            moving = true;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            velocity.0.x -= 1.0;
            *anim_state = PlayerAnimationState::RunLeft;
            moving = true;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity.0.x += 1.0;
            *anim_state = PlayerAnimationState::RunRight;
            moving = true;
        }
        // if mouse.pressed(MouseButton::Left) {
        //     pass
        // }

        if !moving {
            *anim_state = match *anim_state {
                PlayerAnimationState::RunUp => PlayerAnimationState::IdleUp,
                PlayerAnimationState::RunDown => PlayerAnimationState::IdleDown,
                PlayerAnimationState::RunLeft => PlayerAnimationState::IdleLeft,
                PlayerAnimationState::RunRight => PlayerAnimationState::IdleRight,
                _ => anim_state.clone()
            };
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


// fn player_attack {

// }