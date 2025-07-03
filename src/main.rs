
use bevy::prelude::*;
use crate::common::AttackEvent;
use bevy_ecs_tiled::prelude::*;

mod enemy;
mod common;
mod player;
mod animation;
mod collision;
mod combat;
mod minnion;
mod minnions_control;
mod map;
mod ui;

use ui::gui::GuiPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use animation::AnimationPlugin;
use collision::CollisionPlugin;
use combat::CombatPlugin;
use minnion::MinnionsPlugin;
use minnions_control::ControlMinnionsPlugin;
use map::MapPlugin;
use ui::inventory::InventoryPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TiledMapPlugin::default())
        .add_plugins((PlayerPlugin, AnimationPlugin, CollisionPlugin, CombatPlugin, MinnionsPlugin, ControlMinnionsPlugin, MapPlugin, GuiPlugin, InventoryPlugin))
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


