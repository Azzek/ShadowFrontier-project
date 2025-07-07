
use bevy::prelude::*;
use crate::core::common::AttackEvent;
use bevy_ecs_tiled::prelude::*;

mod core;
mod gui;
mod world;
mod player;

#[derive(Resource)]
pub struct DialogWindow {
    pub open: bool
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .insert_resource(DialogWindow{ open: true })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TiledMapPlugin::default())
        .add_plugins((
            player::player::PlayerPlugin,
            core::animation::AnimationPlugin,
            core::collision::CollisionPlugin, 
            core::combat::CombatPlugin, 
            world::minnions::minnion::MinnionsPlugin, 
            world::minnions::control::ControlMinnionsPlugin, 
            world::map::MapPlugin, 
            gui::hud::HudPlugin, 
            gui::inventory::InventoryPlugin, 
            world::npc::NpcPlugin,
            world::enemy::EnemyPlugin
        ))
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


