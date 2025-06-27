use bevy::{
    color::palettes::css::{DARK_CYAN},
    prelude::*,
    text::{FontSmoothing, LineHeight},
};

use crate::common::{Player, Stats};

pub struct UiPlugin;

// Marker component for identifying the HP bar node
#[derive(Component)]
struct Hpbar; 

// Add the setup and update_ui systems to the Update schedule
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, (setup, uptade_ui));
    }
}

// System to update the width of the HP bar based on player's current HP
fn uptade_ui(
    mut query: Query<&mut Node, With<Hpbar>>, // Query for the HP bar UI node
    player_query: Query<&Stats, With<Player>>,              // Query for the player's stats
) {
    if let Ok(mut node) = query.single_mut() {
        if let Ok(p_stats) = player_query.single() {
            if let Ok(stats) = player_query.single() {
                if stats.max_hp > 0 {
                    // Calculate the HP ratio and set the width of the HP bar accordingly
                    let ratio = p_stats.hp as f32 / p_stats.max_hp as f32;
                    node.width = Val::Px(ratio * 370.0); // 370 is the full width when HP is full
                } else {
                    node.width = Val::Px(0.0); // If max HP is zero, set width to 0
                }
            }
        }
    }
}

// System to initialize the UI
fn setup(
    mut commands: Commands,                            // Allows spawning entities
    player_stats_query: Query<&Stats, With<Player>>,   // Query for the player's stats
    asset_server: Res<AssetServer>,                    // Asset server to load fonts
    mut has_spawned: Local<bool>,                      // Local flag to prevent reinitialization
) {
    // If the UI has already been spawned or there is no player, do nothing
    if *has_spawned || player_stats_query.is_empty() {
        return;
    }

    let player_stats = player_stats_query.single().unwrap(); // Get player stats (assumed safe here)
    *has_spawned = true;

    // Load the font for displaying text
    let font = asset_server.load("fonts/Orbitron-Bold.ttf");

    // Define the font style for UI text
    let text_font = TextFont {
        line_height: LineHeight::Px(4.0),
        font_smoothing: FontSmoothing::AntiAliased,
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    let text_justification = JustifyText::Center;

    // Spawn a 2D text element (currently shows placeholder text)
    commands.spawn((
        Text2d::new("Placek Placek"),                      // Placeholder text
        text_font.clone(),                             // Font style
        TextLayout::new_with_justify(text_justification), // Centered text layout
        TextColor(DARK_CYAN.into()),                   // Text color
    ));

    // Spawn the base node for the HP bar background
    commands
        .spawn( Node {
            width: Val::Percent(100.0),                // Full-screen width
            height: Val::Percent(100.0),               // Full-screen height
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        })
        .with_children(|parent| {
            // Spawn the border container for the HP bar
            parent.spawn((
                Node {
                    width: Val::Px(400.0),             // Total bar width
                    height: Val::Px(50.0),             // Bar height
                    ..default()
                },
                Outline {
                    width: Val::Px(4.0),               // Outline width
                    color: DARK_CYAN.into(),           // Outline color
                    offset: Val::Px(10.0),             // Outline offset
                },
                BackgroundColor(Color::srgb(125.0, 125.0, 125.0)), // Background color (gray)
            ));
        })
        .with_children(|parent| {
            // Spawn the actual fill part of the HP bar
            parent.spawn((
                Node {
                    width: Val::Px(player_stats.hp as f32 * 3.5), // Initial width based on HP
                    height: Val::Px(40.0),                         // Fill height
                    position_type: PositionType::Absolute,        // Positioned absolutely inside the container
                    ..default()
                },
                BackgroundColor(Color::srgb(125.0, 0.0, 0.0)), // Fill color (red)
                Hpbar, // Marker component for later access
            ));
        });
}
