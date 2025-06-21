
use bevy::prelude::*;
use crate::common::AttackEvent;

mod enemy;
mod common;
mod player;
mod animation;
mod collision;
mod ui;
mod combat;
mod minnion;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use animation::AnimationPlugin;
use collision::CollisionPlugin;
use ui::UiPlugin;
use combat::CombatPlugin;
use minnion::MinnionsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((PlayerPlugin, AnimationPlugin, CollisionPlugin, CombatPlugin, MinnionsPlugin))
        .add_plugins(UiPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .add_event::<AttackEvent>()
        // .add_systems(Update, debug)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}


fn debug(world: &World) {
    println!("{}", world.entities().len())
}
