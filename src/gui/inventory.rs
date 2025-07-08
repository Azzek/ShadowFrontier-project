use bevy::{
    color::palettes::css::DARK_CYAN,
    input::keyboard,
    prelude::*,
    text::{cosmic_text::ttf_parser::Style, LineHeight},
};

use crate::{core::common::Item, player::player::PlayerGoodies};

/// Marker component for item slots in UI
#[derive(Component)]
pub struct ItemSlot(usize);

/// Marker component for inventory UI root node
#[derive(Component)]
struct InventoryUI;

/// Inventory plugin
#[derive(Component)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, setup_items) // Optional item setup
            .add_systems(Update, toogle_inv); // Add inventory toggle system
    }
}

/// Fills player inventory with dummy items (disabled in plugin)
fn setup_items(mut pg: ResMut<PlayerGoodies>) {
    for i in 0..50 {
        let item = Item {
            id: i.to_string(),
            name: String::from("Zamiatacz"),
            cost: i
        };
        pg.inv.items.push(item);
    }
}

/// Toggles inventory UI with 'I' key
fn toogle_inv(
    keyboard: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut pg: ResMut<PlayerGoodies>,
    ui_query: Query<Entity, With<InventoryUI>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        pg.inv.open = !pg.inv.open;
    }
    // If inventory is being opened
    if pg.inv.open {
        if let Ok(old_inv) = ui_query.single() {
            commands.entity(old_inv).despawn();
        }
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                InventoryUI,
            ))
            .with_children(|parent| {
                // Sidebar panel
                parent
                    .spawn((
                        Node {
                            width: Val::Px(350.),
                            height: Val::Percent(100.),
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Absolute,
                            padding: UiRect {
                                left: Val::Px(30.),
                                right: Val::Px(30.),
                                top: Val::Px(30.),
                                bottom: Val::Px(30.),
                            },
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                        BorderRadius::new(
                            Val::Px(20.),
                            Val::Px(20.),
                            Val::Px(20.),
                            Val::Px(20.),
                        ),
                        BackgroundColor(Color::srgba(200.0, 0.0, 0.0, 0.4)),
                    ))
                    .with_children(|parent| {
                        let slot_img = asset_server.load("inventory/single-slot.png");

                        // Create inventory slots in rows of 4
                        for chunk in pg.inv.items.chunks(4) {
                            // Each row node
                            parent
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Percent(10.),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::FlexStart,
                                    width: Val::Percent(100.),
                                    height: Val::Percent(10.),
                                    ..default()
                                })
                                .with_children(|row| {
                                    // Each slot in the row
                                    for (i, item) in chunk.iter().enumerate() {
                                        row.spawn(Node {
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            justify_items: JustifyItems::Center,
                                            margin: UiRect {
                                                left: Val::Px(5.),
                                                right: Val::Px(5.),
                                                top: Val::Px(25.),
                                                bottom: Val::Px(25.),
                                            },
                                            ..Default::default()
                                        })
                                        .with_children(|node| {
                                            // Slot image
                                            node.spawn((
                                                ImageNode {
                                                    image: slot_img.clone(),
                                                    ..Default::default()
                                                },
                                                Transform::from_scale(Vec3 {
                                                    x: 3.5,
                                                    y: 3.5,
                                                    z: 2.,
                                                }),
                                                ItemSlot(i),
                                            ));

                                            // Item ID text overlay
                                            node.spawn(Node {
                                                position_type: PositionType::Absolute,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..Default::default()
                                            })
                                            .with_children(|id_node| {
                                                id_node.spawn((
                                                    Text::new(item.id.to_string()),
                                                    TextFont {
                                                        font: asset_server
                                                            .load("fonts/Orbitron-Bold.ttf"),
                                                        font_size: 25.0,
                                                        ..default()
                                                    },
                                                ));
                                            });
                                        });
                                    }
                                });
                        }
                    });
                });
        } else {
            // Close inventory UI
            if let Ok(inv) = ui_query.single() {
                commands.entity(inv).despawn();
            }
        }
}
