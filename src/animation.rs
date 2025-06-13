use bevy::prelude::*;
use crate::common::{AnimationIndices, AnimationSet, AnimationState, AnimationTimer};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite)
        .add_systems(Update, change_sprite_texture);
    }
}
// change sprite animation depending on state
fn change_sprite_texture(
    mut query: Query<(&AnimationSet, &AnimationState, &mut Sprite)>
) {
    for (anim_set, anim_state, mut  spr) in query.iter_mut() {
        // Na razie tylko przykład z Idle
        match anim_state {

            AnimationState::Idle =>    {   
                        *spr = Sprite::from_atlas_image(
                            anim_set.idle.0.clone(),
                            TextureAtlas {
                            layout: anim_set.idle.1.clone(),
                            index: spr.texture_atlas.as_ref().map_or(0, |atlas| atlas.index)
                            }
                        )
                    }
                    
            AnimationState::Attack =>   {   
                                            *spr = Sprite::from_atlas_image(
                                                anim_set.attack.0.clone(),
                                                TextureAtlas {
                                                layout: anim_set.attack.1.clone(),
                                                index: spr.texture_atlas.as_ref().map_or(0, |atlas| atlas.index)
                                                }
                                                )
                                            }

            AnimationState::Walk =>    {   
                        *spr = Sprite::from_atlas_image(
                            anim_set.walk.0.clone(),
                            TextureAtlas {
                            layout: anim_set.walk.1.clone(),
                            index: spr.texture_atlas.as_ref().map_or(0, |atlas| atlas.index)
                            }
                        )
                    }
        }

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
