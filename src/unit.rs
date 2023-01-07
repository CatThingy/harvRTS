use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    consts::{SELECTION_COLLISION_GROUP, UNIT_COLLISION_GROUP},
    plot::{Crop, HarvestEvent},
    selection::{Selectable, SelectionIndicator},
};

#[derive(Component, Default)]
struct Unit {
    state: UnitState,
    command: Option<UnitCommand>,
    move_speed: f32,
    aggro_range: f32,
    leash_range: f32,
    last_target_pos: Vec2,
}

#[derive(Default)]
enum UnitState {
    #[default]
    Idle,
    Move,
    Chase,
    Attack,
}

enum UnitCommand {
    Move(Vec2),
    AttackMove(Vec2),
}

pub struct Plugin;

impl Plugin {
    fn process_unit_command(mut q_unit: Query<(&mut Unit, &GlobalTransform)>) {
        for (mut unit, transform) in &mut q_unit {
            if let Some(command) = &unit.command {
                match command {
                    UnitCommand::Move(dest) | UnitCommand::AttackMove(dest) => {
                        if transform.translation().truncate().distance(*dest) <= 4.0 {
                            unit.last_target_pos = *dest;
                            unit.command = None;
                            unit.state = UnitState::Idle;
                        }
                    }
                }
            }
        }
    }

    fn process_unit_state(mut q_unit: Query<(&mut Unit, &GlobalTransform)>) {
        for (mut unit, transform) in &mut q_unit {
            match unit.state {
                UnitState::Idle => {}
                UnitState::Move => {}
                UnitState::Chase => {}
                UnitState::Attack => {}
            }
        }
    }

    fn handle_harvest_event(
        mut cmd: Commands,
        mut ev_harvest: ResMut<Events<HarvestEvent>>,
        assets: Res<AssetServer>,
    ) {
        for harvest in ev_harvest.drain() {
            match harvest.crop {
                Crop::Carrot => {
                    let mut pos = harvest.pos;
                    pos.z = 0.1;
                    cmd.spawn((
                        SpriteBundle {
                            texture: assets.load("carrot_unit.png"),
                            transform: Transform::from_translation(pos),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Velocity::default(),
                        Collider::ball(4.0),
                        LockedAxes::ROTATION_LOCKED_Z,
                        CollisionGroups {
                            memberships: SELECTION_COLLISION_GROUP | UNIT_COLLISION_GROUP,
                            filters: UNIT_COLLISION_GROUP,
                        },
                        Unit::default(),
                        Selectable::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                texture: assets.load("arrow.png"),
                                sprite: Sprite {
                                    anchor: bevy::sprite::Anchor::TopCenter,
                                    ..default()
                                },
                                visibility: Visibility::INVISIBLE,
                                ..default()
                            },
                            SelectionIndicator,
                        ));
                    });
                }
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::handle_harvest_event);
    }
}
