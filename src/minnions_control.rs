use bevy::{ecs::world, input::keyboard::{self, Key}, picking::window, prelude::*, state::commands, text::cosmic_text::ttf_parser::math, transform, window::PrimaryWindow};

use crate::{common::MoveTo, minnion::{Minnion, MinnionMode}};

#[derive(Resource, Default)]
pub struct SelectionBox {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
    pub entity: Option<Entity>, 
}

#[derive(Component)]
struct Selected;

#[derive(Component)]
pub struct SelectionOutline;

pub struct ControlMinnionsPlugin ;

impl Plugin for ControlMinnionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectionBox::default())
        .add_systems(Update, (start_drag_system, update_drag_system, end_drag_system, command_selected_minnions, change_selected_mode));
    }
}

fn start_drag_system(
    mut commands: Commands,
    mut selection: ResMut<SelectionBox>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Get window
        let window = match windows.single() {
            Ok(win) => win,
            Err(_) => return,
        };

        // Get sursor pos
        if let Some(screen_pos) = window.cursor_position() {
            // Get camera and camera transform
            if let Ok((camera, cam_tf)) = q_camera.single() {
                // Convert screen pos on world pos
                if let Ok(world_pos) = camera.viewport_to_world(cam_tf, screen_pos)
                                               .map(|ray| ray.origin.truncate()) {
                    // Set start 
                    selection.start = Some(world_pos);
                    selection.end = Some(world_pos); 


                    let rect = commands
                        .spawn((
                            // white semi-transparent 1×1 sprite, scaled later
                            Sprite {
                                color: Color::linear_rgba(0.3, 0.5, 1.0, 0.2),
                                custom_size: Some(Vec2::ONE),
                                ..Default::default()
                            },
                            Transform {
                                translation: world_pos.extend(0.0),
                                ..Default::default()
                            },
                            GlobalTransform::default()
                        ))
                        .id();

                    selection.entity = Some(rect);
                }
            }
        }
    }
}


fn update_drag_system(
    mut selection: ResMut<SelectionBox>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut query: Query<(&mut Transform, &mut Sprite)>
) {
    if mouse.pressed(MouseButton::Left) {
        // Check if rect exists
        if let (Some(start), Some(react_entity)) = (selection.start, selection.entity) {
            // Get window
            let window = match windows.single()  {
                Ok(win) => win,
                Err(_) => return

            };
            // Get cursor pos if it is in window
            if let Some(screen_pos) = window.cursor_position() {
                // Get camera and camera tranform
                if let Ok((camera, camera_tf)) = q_camera.single() {
                    // Get camera global pos
                    if let Ok(world_pos) = camera
                        .viewport_to_world(camera_tf, screen_pos)
                        .map(|r| r.origin.truncate())
                    {
                        selection.end = Some(world_pos);

                        // Needed informations.
                        
                        // Lower left corner of rectangle
                        let min = start.min(world_pos);
                        // Upper right corner of rectangle
                        let max = start.max(world_pos);
                        // Size of rect
                        let size = max - min;
                        // Center of rect
                        let center = (min + max) / 2.0;
                        // Update rect sprite
                        if let Ok((mut transform, mut sprite)) = query.get_mut(react_entity) {
                            transform.translation = center.extend(0.0);
                            sprite.custom_size = Some(size);
                        }
                    }
                }
            }
        }      
    }
}

fn end_drag_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut selection: ResMut<SelectionBox>,
    mut commands: Commands,
    q_minnions: Query<(Entity, &Transform, &Children), With<Minnion>>,
    q_outlines: Query<Entity, With<SelectionOutline>>,
) {
    if mouse.just_released(MouseButton::Left) {
        if let (Some(start), Some(end)) = (selection.start, selection.end) {
            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);

            for (mn, mn_tf, children) in q_minnions.iter() {
                let tr = mn_tf.translation;
                if tr.x >= min_x && tr.x <= max_x && tr.y >= min_y && tr.y <= max_y {
                    commands.entity(mn).insert(Selected);
                    commands.entity(mn).with_children(|parent| {
                        parent.spawn((
                            Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::new(30.0, 30.0)), 
                                ..default()
                            },
                            Transform::from_xyz(0.0, 0.0, -0.1), 
                            SelectionOutline
                        ));
                    });
                } else {
                    commands.entity(mn).remove::<Selected>();

                   for child in children {
                        if q_outlines.get(*child).is_ok() {
                            commands.entity(*child).despawn();
                        }
                    }
                }
            }
        }

        if let Some(rect) = selection.entity.take() {
            commands.entity(rect).despawn();
        }

        selection.start = None;
        selection.end = None;
    }
}


fn command_selected_minnions (
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_minnions: Query<Entity, (With<Minnion>, With<Selected>)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        let window = match windows.single()  {
                Ok(win) => win,
                Err(_) => return
            };
        if let Some(screen_pos) = window.cursor_position() {
                // Get camera and camera tranform
                if let Ok((camera, camera_tf)) = q_camera.single() {
                    // Get camera global pos
                    if let Ok(world_pos) = camera
                        .viewport_to_world(camera_tf, screen_pos)
                        .map(|r| r.origin.truncate())
                    {
                       for mn in q_minnions {
                            commands.entity(mn).insert(MoveTo { loc: Vec3 { x: world_pos.x, y: world_pos.y, z: 0. } });
                        }
                    }
                }
            }
    }
}

fn change_selected_mode(
    q_minnions: Query<Entity, With<Selected>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        for mn in q_minnions {
            commands.entity(mn).insert(MinnionMode::Neutral);
        }
    }
    if keyboard.just_pressed(KeyCode::KeyB) {
        for mn in q_minnions {
            commands.entity(mn).insert(MinnionMode::Aggresiv);
        }
    }
    if keyboard.just_pressed(KeyCode::KeyV) {
        for mn in q_minnions {
            commands.entity(mn).insert(MinnionMode::Passiv);
        }
    }
}