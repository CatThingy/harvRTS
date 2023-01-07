use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLOT_SIZE: Vec2 = Vec2::new(32.0, 16.0);
pub const PLOT_COLLISION_GROUP: Group = Group::GROUP_31;

pub const PLOT_CIRCLE_RADIUS: f32 = 24.0;

pub const PLOT_CIRCLE_BUTTON_RADIUS: f32 = 4.0;

pub const CARROT_GROW_TIME: f32 = 3.0;
pub const CARROT_DECAY_TIME: f32 = 3.0;
