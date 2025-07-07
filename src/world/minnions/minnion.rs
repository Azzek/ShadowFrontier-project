use std::{ clone, time::Duration };
use crate::{core::common::{Animation, AnimationIndices, AnimationSet, AnimationState, AttackEvent, Collider, HitReactionTimer, MoveTo, Player, Stats, Target}, world::enemy::Enemy};
use bevy::{platform::collections::HashMap, prelude::*, state::commands, window::PrimaryWindow};

pub struct MinnionsPlugin;

#[derive(Component)]
pub struct  Minnion;

#[derive(Resource, Default)]
pub struct SelectionBox {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
}

#[derive(Component)]
struct MinnionAttackTimer {
    timer: Timer,
}


#[derive(Component)]
pub struct MinnionHealthBar;


#[derive(Resource)]
struct MinnionSpawnTimer(Timer);

#[derive(Component, PartialEq)]
pub enum MinnionMode {
    Neutral,
    Aggresiv,
    Passiv
}

impl Plugin for MinnionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MinnionSpawnTimer(Timer::from_seconds(0.125, TimerMode::Once)))
        .add_systems(
            Update, 
            (spawn_minnion, 
                update_hp_bars, 
                hit_reaction, 
                find_enemy_target, 
                drop_target, 
                move_minnions_tow_target, 
                change_animation_state,
                attack
            ));
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
            let idle_tex: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Idle.png");
            let walk_tex: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Walk.png");
            let atk_tex: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Attack01.png");
            let hurt_tex: Handle<Image> = asset_server.load("Soldier/Soldier/Soldier-Hurt.png");
            
            // Load textures and layouts
            let anim_layout   = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None));
            let hurt_layout   = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(100), 3, 1, None, None));

    
            let anim_map = HashMap::from([
                (AnimationState::Idle,   (idle_tex.clone(), anim_layout.clone(), AnimationIndices { first: 0, last: 5 })),
                (AnimationState::Walk,   (walk_tex.clone(), anim_layout.clone(), AnimationIndices { first: 0, last: 5 })),
                (AnimationState::Attack01, (atk_tex.clone(),  anim_layout.clone(), AnimationIndices { first: 0, last: 5 })),
                (AnimationState::Hurt,   (hurt_tex.clone(), hurt_layout.clone(), AnimationIndices { first: 0, last: 2 })),
            ]);

            let animation_set = AnimationSet{ animations: anim_map };

            let mut hit_timer = HitReactionTimer {
                timer: Timer::from_seconds(0.4, TimerMode::Once),
            };

            hit_timer.timer.tick(Duration::from_secs_f32(0.4));

            let animation = Animation {
                    set: animation_set,
                    state: AnimationState::Idle,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    last_state: None
                };

            // Spawn Minion
            let minnion_ent = commands.spawn((
                Sprite::from_atlas_image(
                    idle_tex,
                    TextureAtlas { layout: anim_layout, index: 0 }
                ),
                animation,
                Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3 {
                            x: cursor_pos.x,
                            y: cursor_pos.y,
                            z: 0.0,
                        }),
                Minnion,
                Collider { radius: 22. },
                Stats { hp:100, max_hp:100, attack:25 },
                hit_timer,
                MinnionMode::Neutral,
                MinnionAttackTimer {
                    timer: Timer::from_seconds(0.6, TimerMode::Repeating),
                },
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
    mut query: Query<(Entity, &Stats, &mut HitReactionTimer, &mut MinnionMode), With<Minnion>>,
    
    time: Res<Time>,
) {
    for (enemy, stats, mut reaction_timer,mut mode) in query.iter_mut() {
        reaction_timer.timer.tick(time.delta());

        if reaction_timer.timer.just_finished() && *mode == MinnionMode::Neutral {
            *mode = MinnionMode::Aggresiv;
        }
        if stats.hp <= 0 {
            commands.entity(enemy).despawn();
            continue;
        }
    }
}


// Finds target for minnion 
fn find_enemy_target(
    minnions: Query<(Entity, &Transform, &MinnionMode), (With<Minnion>, (Without<Player>, Without<Enemy>, Without<Target>))>,
    targets: Query<(Entity, &Transform), (With<Enemy>, Without<Minnion>, Without<Player>)>,
    mut commands: Commands,
) {
    for (minnion, minnion_tf, mode) in minnions.iter() {
        if *mode != MinnionMode::Aggresiv {
            continue;
        }
        let mut closest_target: Option<(Entity, f32)> = None;

        for (target, target_tf) in targets.iter() {
            let dist = minnion_tf.translation.distance(target_tf.translation);
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
            commands.entity(minnion).insert(Target { target: ent });
        } else {
            commands.entity(minnion).remove::<Target>();
        }
    }
}


fn drop_target(
    mut query: Query<(Entity, &mut Target, &Transform), With<Minnion>>,
    targets: Query<&Transform>,
    mut commands: Commands,
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


fn move_minnions_tow_target(
    mut minnions: Query<(Entity, &mut Transform, &MinnionAttackTimer, Option<&Target>, Option<&mut MoveTo>), With<Minnion>>,
    targets: Query<&Transform, (Without<Minnion>, Without<Player>)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (mn, mut minnion_tf, attack_timer, maybe_target, maybe_mt) in minnions.iter_mut() {
        // If the enemy has a target assigned
       let move_loc: Option<Vec3> = maybe_mt
        .as_ref()
        .map(|mt| mt.loc)
        .or_else(|| {
            maybe_target.and_then(|target| {
                targets.get(target.target).ok().map(|tf| tf.translation)
            })
        });

        if let Some(move_to) = move_loc {
            if !attack_timer.timer.finished() {
                let direction = (move_to - minnion_tf.translation).normalize_or_zero();
                
                // Movement towards the target
                minnion_tf.translation += direction * time.delta_secs() * 100.0;

                // Rotate the sprite towards the target
                if direction.x.abs() > 0.1 {
                    minnion_tf.scale.x = direction.x.signum() * 4.0;
                }
            }
        }
        if let Some(mt) = maybe_mt {
            if minnion_tf.translation.distance(mt.loc) < 30. {
                commands.entity(mn).remove::<MoveTo>();
            }
        }
    } 
}


fn change_animation_state(
    q: Query<(&HitReactionTimer, &mut Animation, &Transform, Option<&Target>, Option<&MoveTo>, &MinnionMode), With<Minnion>>,
    targets_q: Query<&Transform, (With<Enemy>, Without<Minnion>)>
) {

    for (hit_timer,mut anim, tf,  maybe_target, maybe_mt, mode) in q {
        if !hit_timer.timer.finished() {
            anim.state = AnimationState::Hurt;
            continue;
        }

        if let Some(mt) = maybe_mt {
            if tf.translation.distance(mt.loc) > 30. {
                anim.state = AnimationState::Walk;
                continue;
            }
        }

        if *mode == MinnionMode::Aggresiv {
            if let Some(target) = maybe_target {
                if let Ok(target_tf) = targets_q.get(target.target) {

                    if tf.translation.distance(target_tf.translation) < 110. {
                        anim.state = AnimationState::Attack01;
                        continue;
                    } else {
                        anim.state = AnimationState::Walk;
                        continue;
                    }
                }
            }
        }
        anim.state = AnimationState::Idle;
    }
}


fn attack(
    mut enemy_q: Query<(Entity, &Transform, &mut MinnionAttackTimer, &HitReactionTimer, Option<&Target>, &MinnionMode), With<Minnion>>,
    targets_q: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
    mut attack_events: EventWriter<AttackEvent>,
) {
    for (enemy_ent, enemy_tf, mut cooldown, hit_timer, maybe_target, mode) in enemy_q.iter_mut() {
        if *mode == MinnionMode::Aggresiv {
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
}





