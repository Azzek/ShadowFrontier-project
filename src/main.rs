
use bevy::{
    ecs::schedule::graph::Direction, input::keyboard::{Key, KeyboardInput}, prelude::*, render::texture, sprite, state::commands, tasks::futures_lite::io::Repeat
};
use rand::random_range;


mod enemy;
mod common;
mod player;
mod animation;
mod collision;


use enemy::EnemyPlugin;
use player::PlayerPlugin;
use animation::AnimationPlugin;
use collision::CollisionPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((PlayerPlugin, AnimationPlugin, CollisionPlugin))
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, debug)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}


fn debug(world: &World) {
    println!("{}", world.entities().len())
}
