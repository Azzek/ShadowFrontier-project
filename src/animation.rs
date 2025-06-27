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
            if let Some((_, _, indices)) = animation.set.animations.get(&animation.state) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index >= indices.last {
                        atlas.index = indices.first;
                    } else {
                        atlas.index += 1;
                    }
                }
            }

            animation.timer.reset();
        }
    }
}


fn change_sprite_texture(
    mut query: Query<(&mut Animation, &mut Sprite)>
) {
    for (mut animation, mut sprite) in &mut query {
        
        if animation.last_state == Some(animation.state.clone()) {
            continue;
        }

        if let Some((handle, layout, indices)) = animation.set.animations.get(&animation.state) {
            *sprite = Sprite::from_atlas_image(
                handle.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: indices.first,
                },
            );
        }

        animation.last_state = Some(animation.state.clone());
    }
}

