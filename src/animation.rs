use bevy::prelude::*;
use crate::common::{AnimationTimer, AnimationIndices};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&AnimationIndices, &mut Sprite, &mut AnimationTimer)>) {
    for (indices, mut sprite, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
