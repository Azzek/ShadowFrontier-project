use bevy::prelude::*;
use crate::core::common::Collider;

/// A plugin that handles collision
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // Register the collision detection system in the Update schedule
        app.add_systems(Update, detect_collisions);
    }
}

/// Detects and resolves collisions between entities with `Collider` and `Transform` components
fn detect_collisions(mut query: Query<(Entity, &mut Transform, &Collider)>) {
    // Collect entities data into a vector for comparison
    let entities: Vec<(Entity, Vec3, f32)> = query
        .iter()
        .map(|(e, t, c)| (e, t.translation, c.radius))
        .collect();

    // Iterate over all pairs of entities
    for i in 0..entities.len() {
        let (e1, mut p1, r1) = entities[i];

        for j in (i + 1)..entities.len() {
            let (e2, mut p2, r2) = entities[j];

            // Vector distance between entities
            let delta: Vec3 = p2 - p1;
            // Distanse between entities
            let dist: f32 = delta.length();
            // Minimum allowed distance
            let min_dist: f32 = r1 + r2;

            // Check for collision
            if dist < min_dist {
                // Colliders overlap
                let overlap = min_dist - dist;
                let dir = delta.normalize();

                // Resolve collision by moving entities apart
                p1 -= dir * (overlap * 0.5);
                p2 += dir * (overlap * 0.5);

                // Update the transforms with the new positions
                if let Ok((_, mut t1, _)) = query.get_mut(e1) {
                    t1.translation = p1;
                }
                if let Ok((_, mut t2, _)) = query.get_mut(e2) {
                    t2.translation = p2;
                }
            }
        }
    }
}
