use bevy::{ecs::world, picking::window, prelude::*, state::commands, text::cosmic_text::ttf_parser::math, transform, window::PrimaryWindow};

#[derive(Resource, Default)]
pub struct SelectionBox {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
    pub entity: Option<Entity>, 
}

pub struct ControlMinnionsPlugin ;

impl Plugin for ControlMinnionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectionBox::default())
        .add_systems(Update, (start_drag_system, update_drag_system));
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