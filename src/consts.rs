use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLOT_SIZE: Vec2 = Vec2::new(32.0, 16.0);
pub const PLOT_COLLISION_GROUP: Group = Group::GROUP_32;

pub const PLOT_CIRCLE_RADIUS: f32 = 24.0;

pub const PLOT_CIRCLE_BUTTON_RADIUS: f32 = 4.0;

pub const PLOT_UNLOCK_COST: u32 = 150;

pub const CARROT_GROW_TIME: f32 = 10.0;
pub const CARROT_DECAY_TIME: f32 = 30.0;
pub const CARROT_COST: u32 = 50;
pub const CARROT_COMPOST: u32 = 140;

pub const CLOVER_GROW_TIME: f32 = 3.0;
pub const CLOVER_DECAY_TIME: f32 = 4.0;
pub const CLOVER_COST: u32 = 0;
pub const CLOVER_COMPOST: u32 = 30;

pub const WHEAT_GROW_TIME: f32 = 5.0;
pub const WHEAT_DECAY_TIME: f32 = 10.0;
pub const WHEAT_COST: u32 = 20;
pub const WHEAT_COMPOST: u32 = 60;

// -------------------------------------------------------------------------------------------

pub const CARROT_COUNT: u32 = 3;
pub const CARROT_MOVE_SPEED: f32 = 50.0;
pub const CARROT_AGGRO_RANGE: f32 = 25.0;
pub const CARROT_CHASE_RANGE: f32 = 50.0;
pub const CARROT_LEASH_RANGE: f32 = 100.0;
pub const CARROT_ATTACK_RANGE: f32 = 10.0;
pub const CARROT_ATTACK_SPEED: f32 = 1.0;
pub const CARROT_HEALTH: f32 = 20.0;
pub const CARROT_DAMAGE: f32 = 4.0;

pub const CLOVER_COUNT: u32 = 7;
pub const CLOVER_MOVE_SPEED: f32 = 100.0;
pub const CLOVER_AGGRO_RANGE: f32 = 25.0;
pub const CLOVER_CHASE_RANGE: f32 = 50.0;
pub const CLOVER_LEASH_RANGE: f32 = 100.0;
pub const CLOVER_ATTACK_RANGE: f32 = 10.0;
pub const CLOVER_ATTACK_SPEED: f32 = 1.0;
pub const CLOVER_HEALTH: f32 = 1.0;
pub const CLOVER_DAMAGE: f32 = 0.7;

pub const WHEAT_COUNT: u32 = 5;
pub const WHEAT_MOVE_SPEED: f32 = 75.0;
pub const WHEAT_AGGRO_RANGE: f32 = 40.0;
pub const WHEAT_CHASE_RANGE: f32 = 50.0;
pub const WHEAT_LEASH_RANGE: f32 = 100.0;
pub const WHEAT_ATTACK_RANGE: f32 = 10.0;
pub const WHEAT_ATTACK_SPEED: f32 = 1.0;
pub const WHEAT_HEALTH: f32 = 10.0;
pub const WHEAT_DAMAGE: f32 = 1.0;

pub const APHID_MOVE_SPEED: f32 = 75.0;
pub const APHID_AGGRO_RANGE: f32 = 100.0;
pub const APHID_CHASE_RANGE: f32 = 300.0;
pub const APHID_LEASH_RANGE: f32 = 100.0;
pub const APHID_ATTACK_RANGE: f32 = 10.0;
pub const APHID_ATTACK_SPEED: f32 = 0.5;
pub const APHID_HEALTH: f32 = 5.0;
pub const APHID_DAMAGE: f32 = 0.51;

pub const CATERPILLAR_MOVE_SPEED: f32 = 25.0;
pub const CATERPILLAR_AGGRO_RANGE: f32 = 25.0;
pub const CATERPILLAR_CHASE_RANGE: f32 = 50.0;
pub const CATERPILLAR_LEASH_RANGE: f32 = 100.0;
pub const CATERPILLAR_ATTACK_RANGE: f32 = 16.0;
pub const CATERPILLAR_ATTACK_SPEED: f32 = 2.0;
pub const CATERPILLAR_HEALTH: f32 = 25.0;
pub const CATERPILLAR_DAMAGE: f32 = 7.0;

pub const SELECTION_COLLISION_GROUP: Group = Group::GROUP_31;
pub const UNIT_COLLISION_GROUP: Group = Group::GROUP_1;

pub const FRIENDLY_COLLISION_GROUP: Group = Group::GROUP_30;
pub const ENEMY_COLLISION_GROUP: Group = Group::GROUP_29;

pub const TARGET_MOVEMENT_SLOP: f32 = 16.0;
