use bevy::{
    color::palettes::css::DARK_CYAN,
    prelude::*,
    text::{FontSmoothing, LineHeight},
};

use crate::common::{Player, Stats};

/// Plugin for GUI-related systems
pub struct GuiPlugin;

/// Marker component for identifying the HP bar node
#[derive(Component)]
struct Hpbar;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        // Add GUI setup and update systems
        app.add_systems(Update, (setup, uptade_ui));
    }
}

/// Updates the HP bar width based on the player's current HP
fn uptade_ui(
    mut query: Query<&mut Node, With<Hpbar>>,         // Query for the HP bar UI node
    player_query: Query<&Stats, With<Player>>,        // Query for the player's stats
) {
    if let Ok(mut node) = query.single_mut() {
        if let Ok(p_stats) = player_query.single() {
            if p_stats.max_hp > 0 {
                // Calculate the HP ratio and update the width accordingly
                let ratio = p_stats.hp as f32 / p_stats.max_hp as f32;
                node.width = Val::Px(ratio * 370.0); // Full width = 370px when HP is full
            } else {
                node.width = Val::Px(0.0); // No max HP – hide the bar
            }
        }
    }
}

/// Initializes the GUI (runs once)
fn setup(
    mut commands: Commands,                             // Used to spawn UI entities
    player_stats_query: Query<&Stats, With<Player>>,    // Get the player's stats
    asset_server: Res<AssetServer>,                     // Load font assets
    mut has_spawned: Local<bool>,                       // Prevents re-running this setup
) {
    // Prevent duplicate UI creation or run if player is not ready
    if *has_spawned || player_stats_query.is_empty() {
        return;
    }

    let player_stats = player_stats_query.single().unwrap();
    *has_spawned = true;

    // Load font
    let font = asset_server.load("fonts/Orbitron-Bold.ttf");

    // Define font settings for 2D text
    let text_font = TextFont {
        line_height: LineHeight::Px(4.0),
        font_smoothing: FontSmoothing::AntiAliased,
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    let text_justification = JustifyText::Center;

    // Example: spawn placeholder 2D text
    commands.spawn((
        Text2d::new("Placek Placek"),                       // Example name/text
        text_font.clone(),                                  // Font settings
        TextLayout::new_with_justify(text_justification),   // Center alignment
        TextColor(DARK_CYAN.into()),                        // Color of the text
    ));

    // Background node (screen-filling UI root)
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        })
        .with_children(|parent| {
            // Outer border of the HP bar
            parent.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(50.0),
                    ..default()
                },
                Outline {
                    width: Val::Px(4.0),
                    color: DARK_CYAN.into(),
                    offset: Val::Px(10.0),
                },
                BackgroundColor(Color::srgb(125.0, 125.0, 125.0)), // Gray background
            ));
        })
        .with_children(|parent| {
            // Actual red HP bar (fill)
            parent.spawn((
                Node {
                    width: Val::Px(player_stats.hp as f32 * 3.5), // Initial width
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgb(125.0, 0.0, 0.0)), // Red bar color
                Hpbar, // Marked for updates
            ));
        });
}
