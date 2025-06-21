use bevy::prelude::*;
use crate::common::Animation;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            animate_sprite,
            change_sprite_texture.after(animate_sprite),
        ));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut Sprite)>
) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= animation.indices.last {
                    animation.timer.reset();
                    animation.indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn change_sprite_texture(
    mut query: Query<(&Animation, &mut Sprite), Changed<Animation>>
) {
    for (animation, mut sprite) in &mut query {
        if let Some((handle, layout)) = animation.set.animations.get(&animation.state) {
            *sprite = Sprite::from_atlas_image(
                handle.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: sprite.texture_atlas.as_ref().map_or(0, |atlas| atlas.index),
                },
            );
        }
    }
}
