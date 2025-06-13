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

// Component to switch between animations for enemies and some things..
#[derive(Component)]
pub struct AnimationSet {
    pub idle: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub walk: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub attack: (Handle<Image>, Handle<TextureAtlasLayout>),
}


#[derive(Component, Debug)]
pub enum AnimationState {
    Idle,
    Walk,
    Attack
}


#[derive(Component)]
pub struct Player;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
struct PlayerMinion(MinionType);
