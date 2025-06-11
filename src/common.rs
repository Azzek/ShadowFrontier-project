use bevy::prelude::*;


enum MinionType {
    Soldier
}


#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}


#[derive(Component)]
pub struct Player;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
struct PlayerMinion(MinionType);