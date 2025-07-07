use bevy::{platform::collections::HashMap, prelude::*,};

use crate::{core::common::{Item, Player}, DialogWindow};

/// NPC roles (can be either a general NPC or a shopkeeper)
#[derive(PartialEq, Clone)]
enum NpcRole {
    Shop,
    Npc,
}

/// Marker component for identifying the dialog UI of a shop
#[derive(Component)]
struct ShopDialog;

/// Container for NPC offers (shop inventory), wrapped in a component
#[derive(Clone, Component)]
struct NpcOffers {
    map: HashMap<String, Item>,
}

/// Main NPC component, attached to entities
#[derive(Component, Clone)]
struct Npc {
    name: String,
    loc: Vec3,
    sprite_path: String,
    role: NpcRole,
    offer: Option<NpcOffers>,
}

/// Resource storing all NPC definitions
#[derive(Resource)]
struct NpcList {
    list: HashMap<String, Npc>,
}

/// Bevy plugin that registers systems related to NPCs
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            // Load and spawn NPCs on startup
            .add_systems(Startup, (load_npcs, spawn_npcs).chain())
            // Update system for shop interaction
            .add_systems(Update, npc_shop_interaction);
    }
}

/// Loads NPC definitions and inserts them as a resource
fn load_npcs(mut commands: Commands) {
    let npc_list = HashMap::from([(
        String::from("Zdzichu"),
        Npc {
            name: String::from("Zdzichu"),
            loc: Vec3 { x: 0., y: 0., z: 3. },
            sprite_path: String::from("Player/Sprites/IDLE/idle_down.png"),
            role: NpcRole::Shop,
            offer: Some(NpcOffers {
                map: HashMap::from([
                    (String::from("1"), Item { id: String::from("1"), name: String::from("Zadymiacz") }),
                    (String::from("2"), Item { id: String::from("2"), name: String::from("Zadymiacz") }),
                    (String::from("3"), Item { id: String::from("3"), name: String::from("Zadymiacz") }),
                ]),
            }),
        },
    )]);

    commands.insert_resource(NpcList { list: npc_list });
}

/// Spawns NPC entities into the world based on the NpcList resource
fn spawn_npcs(
    mut commands: Commands,
    npcs: Res<NpcList>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (_name, npc_data) in npcs.list.iter() {
        // Load sprite and animation layout
        let image_handle = asset_server.load(&npc_data.sprite_path);
        let layout_8fps = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(96),
            8,
            1,
            None,
            None,
        ));

        // Spawn the NPC entity with sprite and transform
        let entity = commands.spawn((
            Sprite::from_atlas_image(
                image_handle,
                TextureAtlas {
                    layout: layout_8fps,
                    index: 0,
                },
            ),
            Transform {
                translation: npc_data.loc,
                scale: Vec3::splat(2.5),
                ..default()
            },
            npc_data.clone(), // Clone the Npc component and attach it
        ))
        .id();

        // If the NPC has shop offers, attach them as a component
        if let Some(offers) = &npc_data.offer {
            commands.entity(entity).insert(offers.clone());
        }
    }
}

/// System that triggers when player presses `E` near an NPC with offers
fn npc_shop_interaction(
    keyboard: Res<ButtonInput<KeyCode>>,
    q_npc: Query<(&Npc, &Transform, Option<&NpcOffers>)>,
    q_shop: Query<Entity, With<ShopDialog>>,
    q_player: Query<&Transform, With<Player>>,
    mut diag_window: ResMut<DialogWindow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Trigger only when E key is just pressed
    if keyboard.just_pressed(KeyCode::KeyE) {
        for p_tf in q_player.iter() {
            for (npc, npc_tf, maybe_offers) in q_npc.iter() {
                // Check proximity between player and NPC
                if p_tf.translation.distance(npc_tf.translation) < 150.0 {
                    diag_window.open = !diag_window.open;

                    if diag_window.open {
                        // Spawn shop dialog background
                        commands.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                height: Val::Percent(100.0),
                                width: Val::Percent(100.0),
                                ..Default::default()
                            },
                            ShopDialog,
                        ))
                        .with_children(|container| {
                            // Shop UI panel
                            container.spawn((
                                Node {
                                    width: Val::Percent(60.0),
                                    height: Val::Percent(80.0),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Start,
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                BackgroundColor(Color::srgba(125.0, 125.0, 125.0, 0.5)),
                                BorderRadius::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                ),
                            ))
                            .with_children(|container| {
                                if let Some(offers) = maybe_offers {
                                    let slot_image_handle = asset_server.load("inventory/single-slot.png");

                                    // Convert HashMap values to a Vec for chunking
                                    let items: Vec<_> = offers.map.values().collect();

                                    // Chunk the items into rows of 8
                                    for chunk in items.chunks(8) {
                                        // Each chunk is one row (horizontal Node)
                                        container.spawn(Node {
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            align_items: AlignItems::Center,
                                            width: Val::Percent(100.),
                                            height: Val::Px(96.),
                                            padding: UiRect {
                                                left: Val::Px(50.),
                                                right: Val::Px(50.),
                                                top: Val::Px(50.),
                                                bottom: Val::Px(50.),
                                            },
                                            ..Default::default()
                                        })
                                        .with_children(|row| {
                                            for _item in chunk {
                                                // Each slot in the row
                                                row.spawn(Node {
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..Default::default()
                                                })
                                                .with_children(|slot| {
                                                    slot.spawn((
                                                        ImageNode {
                                                            image: slot_image_handle.clone(),
                                                            ..Default::default()
                                                        },
                                                        Transform::from_scale(Vec3::splat(3.5))
                                                    ));
                                                });
                                            }
                                        });
                                    }
                                }
                            });
                        });
                    } else {
                        // If shop was open and now needs closing, despawn dialog
                        if let Ok(shop) = q_shop.single() {
                            commands.entity(shop).despawn();
                        }
                    }
                }
            }
        }
    }
}
