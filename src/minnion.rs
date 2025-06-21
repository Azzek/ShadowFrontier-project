use std::{collections::HashMap, time::Duration};
use crate::common::{Animation, AnimationIndices, AnimationSet, AnimationState, Collider, Stats, HitReactionTimer};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct MinnionsPlugin;

#[derive(Component)]
pub struct  Minnion;


#[derive(Component)]
enum Minnion_mode {
    Neutral,
    Passiv,
    Aggresiv
}


#[derive(Component)]
pub struct MinnionHealthBar;


#[derive(Resource)]
struct MinnionSpawnTimer(Timer);


impl Plugin for MinnionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MinnionSpawnTimer(Timer::from_seconds(0.125, TimerMode::Once)))
        .add_systems(Update, (spawn_minnion, update_hp_bars, hit_reaction));
    }
}


fn spawn_minnion(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut spawn_timer: ResMut<MinnionSpawnTimer>,
    time: Res<Time>
) {
    // Get window
    let window = match q_windows.single() {
        Ok(window) => window,
        Err(_) => return,
    };

    // Get cursor position in screen space
    let Some(screen_pos) = window.cursor_position() else {
        return;
    };

    // Get the camera and its transform
    let (camera, camera_transform) = match q_camera.single() {
        Ok(cam) => cam,
        Err(_) => return,
    };

    // Convert screen space to world space
    let world_pos = camera.viewport_to_world(camera_transform, screen_pos)
        .map(|ray| ray.origin.truncate());

    spawn_timer.0.tick(time.delta());
    
    // Spawn Minnion if key is pressed and timer is finished
    if keyboard.pressed(KeyCode::KeyP) && spawn_timer.0.finished() {
        spawn_timer.0.reset();

        if let Ok(cursor_pos) = world_pos {
            let soldier_idle: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Idle.png");
            let soldier_walk: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Walk.png");
            let soldier_attack01: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Attack01.png");
            
            // Load textures and layouts
            let anim_layout   = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None));

            let mut anim_map = HashMap::new();
            anim_map.insert(AnimationState::Idle, (soldier_idle.clone(), anim_layout.clone()));
            anim_map.insert(AnimationState::Walk, (soldier_walk, anim_layout.clone()));
            anim_map.insert(AnimationState::Attack, (soldier_attack01, anim_layout.clone()));

            let animation_indices = AnimationIndices { first: 0, last: 5 };

            let animation_set = AnimationSet{ animations: anim_map };

            let mut hit_timer = HitReactionTimer {
                timer: Timer::from_seconds(0.4, TimerMode::Once),
            };

            hit_timer.timer.tick(Duration::from_secs_f32(0.4));

            let animation = Animation {
                    set: animation_set,
                    indices: animation_indices,
                    state: AnimationState::Idle,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating)
                };

            // Spawn Minion
            let minnion_ent = commands.spawn((
                Sprite::from_atlas_image(
                    soldier_idle,
                    TextureAtlas { layout: anim_layout, index: 0 }
                ),
                animation,
                Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3 {
                            x: cursor_pos.x,
                            y: cursor_pos.y,
                            z: 0.0,
                        }),
                Minnion,
                Collider { radius: 50. },
                Stats { hp:100, max_hp:100, attack:25 },
                hit_timer
            )).id();

            let hp_bar = Sprite {
                    color: Color::srgb(125.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(100. / 4., 1.0)),
                    ..default()
                };
                
            

            commands.entity(minnion_ent)
            .with_children(| parent | {
                parent.spawn((hp_bar, MinnionHealthBar, Transform::from_xyz(0., 15., 0.)));
            });
        }
    }
    
}


fn update_hp_bars(
    parents: Query<(&Stats, &Children)>,
    mut bars: Query<&mut Sprite, With<MinnionHealthBar>>,
) {
    for (stats, children) in &parents {
        println!("{}", stats.hp);
        for child in children.iter() {
            if let Ok(mut sprite) = bars.get_mut(child) {
                let health_ratio = stats.hp as f32 / stats.max_hp as f32;
                sprite.custom_size = Some(Vec2::new((stats.max_hp as f32 / 4.0) * health_ratio, 1.0));
            }
        }
    }
}


fn hit_reaction(
    mut commands: Commands,
    mut query: Query<(Entity, &Stats, &mut HitReactionTimer, &mut Animation), With<Minnion>>,
    time: Res<Time>,
) {
    for (enemy, stats, mut reaction_timer, mut animation) in query.iter_mut() {
        reaction_timer.timer.tick(time.delta());

        if stats.hp <= 0 {
            commands.entity(enemy).despawn();
            continue;
        }
    }
}