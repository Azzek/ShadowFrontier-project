use bevy::{platform::collections::HashMap, prelude::*};


enum MinionType {
    Soldier
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);


#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}


#[derive(Component)]
pub struct HitReactionTimer {
    pub timer: Timer,
}


#[derive(Component)]
pub struct InvincibilityTimer  {
    pub timer: Timer,
}


#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}


#[derive(Component)]
pub struct AnimationSet {
pub animations: HashMap<AnimationState, (Handle<Image>, Handle<TextureAtlasLayout>, AnimationIndices)>,
}


#[derive(Component)]
pub struct Animation {
    pub set: AnimationSet,
    pub state: AnimationState,
    pub last_state: Option<AnimationState>, // <— nowe pole!
    pub timer: Timer,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    // 2 directions
    Idle,
    Walk,
    Attack01,
    Attack02,
    Hurt,

    // 4 directions
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,

    RunUp,
    RunDown,
    RunLeft,
    RunRight,

    WalkUp,
    WalkDown,
    WalkRight,
    WalkLeft,

    AttackUp,
    AttackDown,
    AttackLeft,
    AttackRight,
}


#[derive(Component)]
pub struct Player;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
struct PlayerMinion(MinionType);

#[derive(Component)]
pub struct Target {
    pub target:Entity
}

#[derive(Component, Clone, Copy)]
pub struct MoveTo {
    pub loc: Vec3
}

#[derive(Component)]
pub struct Stats{
    pub hp: i32,
    pub max_hp: i32,
    pub  attack: i32
}


#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
}