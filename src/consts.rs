use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLOT_SIZE: Vec2 = Vec2::new(32.0, 16.0);
pub const PLOT_COLLISION_GROUP: Group = Group::GROUP_32;

pub const PLOT_CIRCLE_RADIUS: f32 = 24.0;

pub const PLOT_CIRCLE_BUTTON_RADIUS: f32 = 4.0;

pub const PLOT_UNLOCK_COST: u32 = 150;


pub const CARROT_GROW_TIME: f32 = 0.1;
pub const CARROT_DECAY_TIME: f32 = 3.0;
pub const CARROT_COST: u32 = 50;
pub const CARROT_COMPOST: u32 = 60;

pub const CARROT_MOVE_SPEED: f32 = 100.0;
pub const CARROT_AGGRO_RANGE: f32 = 25.0;
pub const CARROT_CHASE_RANGE: f32 = 50.0;
pub const CARROT_LEASH_RANGE: f32 = 100.0;
pub const CARROT_ATTACK_RANGE: f32 = 10.0;
pub const CARROT_ATTACK_SPEED: f32 = 1.0;
pub const CARROT_HEALTH: f32 = 20.0;
pub const CARROT_DAMAGE: f32 = 2.0;

pub const ENEMY_MOVE_SPEED: f32 = 100.0;
pub const ENEMY_AGGRO_RANGE: f32 = 25.0;
pub const ENEMY_CHASE_RANGE: f32 = 50.0;
pub const ENEMY_LEASH_RANGE: f32 = 100.0;
pub const ENEMY_ATTACK_RANGE: f32 = 10.0;
pub const ENEMY_ATTACK_SPEED: f32 = 1.0;
pub const ENEMY_HEALTH: f32 = 5.0;
pub const ENEMY_DAMAGE: f32 = 1.0;

pub const SELECTION_COLLISION_GROUP: Group = Group::GROUP_31;
pub const UNIT_COLLISION_GROUP: Group = Group::GROUP_1;

pub const FRIENDLY_COLLISION_GROUP: Group = Group::GROUP_30;
pub const ENEMY_COLLISION_GROUP: Group = Group::GROUP_29;

pub const TARGET_MOVEMENT_SLOP: f32 = 16.0;
