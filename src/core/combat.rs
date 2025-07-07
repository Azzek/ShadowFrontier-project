use bevy::prelude::*;
use crate::core::common::{HitReactionTimer, Stats, AttackEvent};

/// Plugin responsible for handling combat-related systems, like applying damage
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_attack_events);
    }
}

/// System that processes `AttackEvent`s and reduces the HP of the targeted entity
fn handle_attack_events(
    mut events: EventReader<AttackEvent>,   // Reads all attack events for the current frame
    mut query: Query<(&mut Stats, &mut HitReactionTimer)>,           // Query to access the mutable stats of entities
) {
    // Iterate over all attack events triggered this frame
    for event in events.read() {
        // Attempt to get the target's Stats component using the entity ID from the event
        if let Ok((mut target_stats, mut reaction_timer)) = query.get_mut(event.target) {
            // Apply the damage by subtracting from current HP
            target_stats.hp -= event.damage;
            reaction_timer.timer.reset()
        }
    }
}
