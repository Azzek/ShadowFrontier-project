use bevy::{color::palettes::css::DARK_CYAN, input::keyboard, prelude::*, text::{cosmic_text::ttf_parser::Style, LineHeight}};


#[derive(Resource, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub open: bool,
}


#[derive(Component)]
pub struct ItemSlot(usize);

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[derive(Component)]
struct InventoryUI; 


#[derive(Component)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.
        init_resource::<Inventory>()
        .add_systems(Startup, setup_items)
        .add_systems(Update, (toogle_inv, debug_inv));
    }
}

fn setup_items(mut inv: ResMut<Inventory>) {
    for i in 0..50 {
        let item = Item {
        id: i,
        name: String::from("Zamiatacz")
    };
    inv.items.push(item);
    }
    
}

fn toogle_inv(
    keyboard: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    ui_query: Query<Entity, With<InventoryUI>>,
    asset_server: Res<AssetServer>
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        inventory.open = !inventory.open;

        if inventory.open {
            commands.spawn( (Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            position_type: PositionType::Absolute,
            ..default()
            },
            InventoryUI
            ))
            .with_children( |parent| {
                parent.spawn((Node {
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

                    // Tworzymy sloty po 5 w wierszu
                    for chunk in inventory.items.chunks(4) {
                        // Wiersz slotów
                        parent.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Percent(25.),
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::FlexStart,
                            width: Val::Percent(100.),
                            height: Val::Percent(10.),
                            ..default()
                        }).with_children(|row| {
                            for (i, item) in chunk.iter().enumerate() {
                                
                                row.spawn((
                                    ImageNode {
                                        image: slot_img.clone(),
                                        ..Default::default()
                                    },
                                    Transform::from_scale(Vec3 { x: 3.5, y: 3.5, z: 2. }),
                                    ItemSlot(i),
                                    
                                ))
                                .with_children(|slot| {
                                    slot.spawn((
                                        Text::new(item.id.to_string()),
                                        TextFont {
                                            font: asset_server.load("fonts/Orbitron-Bold.ttf"),
                                            font_size: 3.0,
                                            ..default()
                                        },
                                        
                                    ));
                                });
                            }
                        });
                    }
                });    
            });
        } else {
            if let Ok(inv) = ui_query.single() {
                commands.entity(inv).despawn();
            }
        }
        
    }
}


fn debug_inv(q: Query<&Transform, With<ItemSlot>>) {
    for i in q {
        // println!("{}, {}", i.translation.x, i.tra);
    }
}