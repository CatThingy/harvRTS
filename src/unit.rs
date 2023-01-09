use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::*,
    game::Rose,
    health::{Dead, Health, HealthBar, HealthChange},
    plot::{Crop, HarvestEvent},
    selection::{HoverIndicator, Selectable, SelectionIndicator},
    utils::{Bar, MousePosition, PlaySound},
    GameState,
};

#[derive(Component, Default)]
pub struct Unit {
    state: UnitState,
    command: Option<UnitCommand>,
    move_speed: f32,
    aggro_range: f32,
    chase_range: f32,
    attack_range: f32,
    leash_range: f32,
    attack_timer: Timer,
    damage: f32,
    last_target_pos: Vec2,
    leash_pos: Vec2,
}

impl Unit {
    pub fn new(
        move_speed: f32,
        aggro_range: f32,
        chase_range: f32,
        attack_range: f32,
        leash_range: f32,
        attack_speed: f32,
        damage: f32,
    ) -> Self {
        Unit {
            move_speed,
            aggro_range,
            chase_range,
            attack_range,
            leash_range,
            attack_timer: Timer::from_seconds(attack_speed, TimerMode::Once),
            damage,
            ..default()
        }
    }
}

#[derive(Default)]
enum UnitState {
    #[default]
    Idle,
    Move(Vec2),
    Chase(Entity),
    Attack(Entity),
}

#[derive(Clone)]
enum UnitCommand {
    Move(Vec2),
    AttackMove(Vec2),
}

trait Side {
    const ATTACKS_GROUP: Group;
}

#[derive(Component)]
pub struct Enemy;

impl Side for Enemy {
    const ATTACKS_GROUP: Group = FRIENDLY_COLLISION_GROUP;
}

#[derive(Component, Clone)]
pub struct Friendly;

impl Side for Friendly {
    const ATTACKS_GROUP: Group = ENEMY_COLLISION_GROUP;
}

pub struct Plugin;

impl Plugin {
    fn update_unit_state<T: Side + Component>(
        rapier_ctx: Res<RapierContext>,
        mut q_unit: Query<(&mut Unit, &GlobalTransform), With<T>>,
        q_transform: Query<(&GlobalTransform, Option<&Rose>), Without<Dead>>,
    ) {
        for (mut unit, transform) in &mut q_unit {
            let unit_pos = transform.translation().truncate();
            if let Some(command) = &unit.command {
                match command {
                    UnitCommand::Move(dest) => {
                        if unit_pos.distance(*dest) <= TARGET_MOVEMENT_SLOP {
                            let dest = *dest;
                            unit.last_target_pos = dest;
                            unit.leash_pos = dest;
                            unit.command = None;
                            unit.state = UnitState::Idle;
                            continue;
                        }
                        unit.state = UnitState::Move(*dest);
                    }
                    UnitCommand::AttackMove(dest) => {
                        let dest = *dest;
                        if unit_pos.distance(dest) <= TARGET_MOVEMENT_SLOP {
                            unit.last_target_pos = dest;
                            unit.leash_pos = dest;
                            unit.command = None;
                            unit.state = UnitState::Idle;
                            continue;
                        }

                        match unit.state {
                            UnitState::Idle => {
                                unit.state = UnitState::Move(dest);
                            }
                            UnitState::Move(_) => {
                                if unit_pos.distance(dest) <= TARGET_MOVEMENT_SLOP {
                                    unit.last_target_pos = dest;
                                    unit.leash_pos = dest;
                                    unit.command = None;
                                    unit.state = UnitState::Idle;
                                    continue;
                                }
                                let mut min_target = None;

                                rapier_ctx.intersections_with_shape(
                                    unit_pos,
                                    0.0,
                                    &Collider::ball(unit.chase_range),
                                    QueryFilter::new().groups(InteractionGroups {
                                        memberships: UNIT_COLLISION_GROUP.bits().into(),
                                        filter: T::ATTACKS_GROUP.bits().into(),
                                    }),
                                    |e| {
                                        if let Ok((target, rose)) = q_transform.get(e) {
                                            let target_pos = target.translation().truncate();
                                            let dist = target_pos.distance(unit_pos);

                                            if target_pos.distance(unit.leash_pos)
                                                < unit.leash_range
                                            {
                                                match min_target {
                                                    Some((_, old_dist, is_rose)) => {
                                                        if is_rose || old_dist > dist {
                                                            min_target =
                                                                Some((e, dist, rose.is_some()))
                                                        }
                                                    }
                                                    None => {
                                                        min_target = Some((e, dist, rose.is_some()))
                                                    }
                                                }
                                            }
                                        }
                                        true
                                    },
                                );

                                if let Some((e, _, _)) = min_target {
                                    unit.last_target_pos = dest;
                                    unit.leash_pos = unit_pos;
                                    unit.state = UnitState::Chase(e);
                                } else {
                                    unit.state = UnitState::Move(dest);
                                }
                            }
                            UnitState::Chase(entity) => match q_transform.get(entity) {
                                Ok((p, _)) => {
                                    let target_pos = p.translation().truncate();
                                    let pos = unit_pos;

                                    let distance = pos.distance(target_pos);

                                    let leash_distance = unit.leash_pos.distance(pos);

                                    if distance > unit.chase_range
                                        || leash_distance > unit.leash_range
                                    {
                                        unit.state = UnitState::Move(unit.last_target_pos);
                                    } else {
                                        rapier_ctx.intersections_with_shape(
                                            unit_pos,
                                            0.0,
                                            &Collider::ball(unit.attack_range),
                                            QueryFilter::new().groups(InteractionGroups {
                                                memberships: UNIT_COLLISION_GROUP.bits().into(),
                                                filter: T::ATTACKS_GROUP.bits().into(),
                                            }),
                                            |e| {
                                                if entity == e {
                                                    unit.state = UnitState::Attack(entity);
                                                    return false;
                                                }
                                                return true;
                                            },
                                        );
                                    }
                                }
                                Err(_) => {
                                    unit.state = UnitState::Move(unit.last_target_pos);
                                }
                            },
                            UnitState::Attack(e) => {
                                if unit.attack_timer.finished() {
                                    unit.attack_timer.reset();
                                    unit.state = UnitState::Chase(e);
                                }
                            }
                        }
                    }
                }
            } else {
                match unit.state {
                    UnitState::Idle => {
                        let mut min_target = None;
                        rapier_ctx.intersections_with_shape(
                            unit_pos,
                            0.0,
                            &Collider::ball(unit.aggro_range),
                            QueryFilter::new().groups(InteractionGroups {
                                memberships: UNIT_COLLISION_GROUP.bits().into(),
                                filter: T::ATTACKS_GROUP.bits().into(),
                            }),
                            |e| {
                                if let Ok((target, rose)) = q_transform.get(e) {
                                    let target_pos = target.translation().truncate();
                                    let dist = target_pos.distance(unit_pos);

                                    if target_pos.distance(unit.leash_pos) < unit.leash_range {
                                        match min_target {
                                            Some((_, old_dist, is_rose)) => {
                                                if is_rose || old_dist > dist {
                                                    min_target = Some((e, dist, rose.is_some()))
                                                }
                                            }
                                            None => min_target = Some((e, dist, rose.is_some())),
                                        }
                                    }
                                }
                                true
                            },
                        );

                        if let Some((e, _, _)) = min_target {
                            unit.leash_pos = unit_pos;
                            unit.state = UnitState::Chase(e);
                        }
                    }
                    UnitState::Move(dest) => {
                        if unit_pos.distance(dest) <= TARGET_MOVEMENT_SLOP {
                            unit.last_target_pos = unit_pos;
                            unit.leash_pos = unit_pos;
                            unit.command = None;
                            unit.state = UnitState::Idle;
                            continue;
                        }

                        let mut min_target = None;

                        rapier_ctx.intersections_with_shape(
                            unit_pos,
                            0.0,
                            &Collider::ball(unit.aggro_range),
                            QueryFilter::new().groups(InteractionGroups {
                                memberships: UNIT_COLLISION_GROUP.bits().into(),
                                filter: T::ATTACKS_GROUP.bits().into(),
                            }),
                            |e| {
                                if let Ok((target, rose)) = q_transform.get(e) {
                                    let target_pos = target.translation().truncate();
                                    let dist = target_pos.distance(unit_pos);

                                    if target_pos.distance(unit.leash_pos) < unit.leash_range {
                                        match min_target {
                                            Some((_, old_dist, is_rose)) => {
                                                if is_rose || old_dist > dist {
                                                    min_target = Some((e, dist, rose.is_some()))
                                                }
                                            }
                                            None => min_target = Some((e, dist, rose.is_some())),
                                        }
                                    }
                                }
                                true
                            },
                        );

                        if let Some((e, _, _)) = min_target {
                            unit.state = UnitState::Chase(e);
                        }
                    }
                    UnitState::Chase(entity) => match q_transform.get(entity) {
                        Ok((p, _)) => {
                            let target_pos = p.translation().truncate();
                            let pos = unit_pos;

                            let distance = pos.distance(target_pos);

                            let leash_distance = unit.leash_pos.distance(pos);

                            if distance > unit.aggro_range || leash_distance > unit.leash_range {
                                unit.state = UnitState::Move(unit.leash_pos);
                            } else {
                                rapier_ctx.intersections_with_shape(
                                    unit_pos,
                                    0.0,
                                    &Collider::ball(unit.attack_range),
                                    QueryFilter::new().groups(InteractionGroups {
                                        memberships: UNIT_COLLISION_GROUP.bits().into(),
                                        filter: T::ATTACKS_GROUP.bits().into(),
                                    }),
                                    |e| {
                                        if entity == e {
                                            unit.state = UnitState::Attack(entity);
                                            return false;
                                        }
                                        return true;
                                    },
                                );
                            }
                        }
                        Err(_) => {
                            unit.state = UnitState::Move(unit.leash_pos);
                        }
                    },
                    UnitState::Attack(e) => {
                        if unit.attack_timer.finished() {
                            unit.attack_timer.reset();
                            unit.state = UnitState::Chase(e);
                        }
                    }
                }
            }
        }
    }

    fn process_unit_state(
        mut q_unit: Query<(&mut Velocity, &mut Unit, &GlobalTransform)>,
        q_transform: Query<&GlobalTransform>,
        time: Res<Time>,
        mut damage: EventWriter<HealthChange>,
    ) {
        for (mut velocity, mut unit, transform) in &mut q_unit {
            unit.attack_timer.tick(time.delta());
            match unit.state {
                UnitState::Idle => {}
                UnitState::Move(dest) => {
                    velocity.linvel = (dest - transform.translation().truncate())
                        .normalize_or_zero()
                        * unit.move_speed;
                }
                UnitState::Chase(target) => {
                    if let Ok(target) = q_transform.get(target) {
                        velocity.linvel = (target.translation() - transform.translation())
                            .truncate()
                            .normalize_or_zero()
                            * unit.move_speed;
                    };
                }
                UnitState::Attack(entity) => {
                    if unit.attack_timer.finished() {
                        damage.send(HealthChange {
                            target: entity,
                            amount: -unit.damage,
                        });
                    }
                }
            }
        }
    }

    fn handle_harvest_event(
        mut cmd: Commands,
        mut ev_harvest: ResMut<Events<HarvestEvent>>,
        mut ev_sound: EventWriter<PlaySound>,
        assets: Res<AssetServer>,
    ) {
        let rng = fastrand::Rng::default();
        for harvest in ev_harvest.drain() {
            ev_sound.send(PlaySound("snip.ogg".to_string()));
            let common = (
                RigidBody::Dynamic,
                Velocity::default(),
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                LockedAxes::ROTATION_LOCKED_Z,
                CollisionGroups {
                    memberships: SELECTION_COLLISION_GROUP
                        | UNIT_COLLISION_GROUP
                        | FRIENDLY_COLLISION_GROUP,
                    filters: UNIT_COLLISION_GROUP,
                },
                Damping {
                    linear_damping: 20.0,
                    angular_damping: 0.0,
                },
                Selectable::default(),
                Friendly,
                harvest.crop.clone(),
            );

            let mut pos = harvest.pos;
            pos.z = 0.1;
            match harvest.crop {
                Crop::Carrot => {
                    for _ in 0..CARROT_COUNT {
                        let offset =
                            (Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5) * PLOT_SIZE).extend(0.0);
                        cmd.spawn((
                            SpriteBundle {
                                texture: assets.load("carrot_unit.png"),
                                transform: Transform::from_translation(pos + offset),
                                sprite: Sprite {
                                    anchor: bevy::sprite::Anchor::BottomCenter,
                                    ..default()
                                },
                                ..default()
                            },
                            Collider::ball(4.0),
                            Unit::new(
                                CARROT_MOVE_SPEED,
                                CARROT_AGGRO_RANGE,
                                CARROT_CHASE_RANGE,
                                CARROT_ATTACK_RANGE,
                                CARROT_LEASH_RANGE,
                                CARROT_ATTACK_SPEED,
                                CARROT_DAMAGE,
                            ),
                            Health::new(CARROT_HEALTH),
                            common.clone(),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    texture: assets.load("arrow.png"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -5.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        anchor: bevy::sprite::Anchor::TopCenter,
                                        color: Color::YELLOW,
                                        ..default()
                                    },
                                    visibility: Visibility::INVISIBLE,
                                    ..default()
                                },
                                SelectionIndicator,
                            ));
                            parent.spawn((
                                SpriteBundle {
                                    texture: assets.load("arrow.png"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -5.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        anchor: bevy::sprite::Anchor::TopCenter,
                                        ..default()
                                    },
                                    visibility: Visibility::INVISIBLE,
                                    ..default()
                                },
                                HoverIndicator,
                            ));
                            parent.spawn((
                                SpriteBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -4.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(2.0, 1.0)),
                                        color: Color::RED,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Bar {
                                    value: CARROT_HEALTH,
                                    max: CARROT_HEALTH,
                                    size: 10.0,
                                },
                                HealthBar,
                            ));
                        });
                    }
                }
                Crop::Clover => {
                    for _ in 0..CLOVER_COUNT {
                        let offset =
                            (Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5) * PLOT_SIZE).extend(0.0);
                        cmd.spawn((
                            SpriteBundle {
                                texture: assets.load("clover_unit.png"),
                                transform: Transform::from_translation(pos + offset),
                                sprite: Sprite {
                                    anchor: bevy::sprite::Anchor::BottomCenter,
                                    ..default()
                                },
                                ..default()
                            },
                            Collider::ball(2.0),
                            Unit::new(
                                CLOVER_MOVE_SPEED,
                                CLOVER_AGGRO_RANGE,
                                CLOVER_CHASE_RANGE,
                                CLOVER_ATTACK_RANGE,
                                CLOVER_LEASH_RANGE,
                                CLOVER_ATTACK_SPEED,
                                CLOVER_DAMAGE,
                            ),
                            Health::new(CLOVER_HEALTH),
                            common.clone(),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    texture: assets.load("arrow.png"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -5.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        anchor: bevy::sprite::Anchor::TopCenter,
                                        color: Color::YELLOW,
                                        ..default()
                                    },
                                    visibility: Visibility::INVISIBLE,
                                    ..default()
                                },
                                SelectionIndicator,
                            ));
                            parent.spawn((
                                SpriteBundle {
                                    texture: assets.load("arrow.png"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -5.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        anchor: bevy::sprite::Anchor::TopCenter,
                                        ..default()
                                    },
                                    visibility: Visibility::INVISIBLE,
                                    ..default()
                                },
                                HoverIndicator,
                            ));
                            parent.spawn((
                                SpriteBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -4.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(2.0, 1.0)),
                                        color: Color::RED,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Bar {
                                    value: CLOVER_HEALTH,
                                    max: CLOVER_HEALTH,
                                    size: 10.0,
                                },
                                HealthBar,
                            ));
                        });
                    }
                }

                Crop::Wheat => {
                    for _ in 0..WHEAT_COUNT {
                        let offset =
                            (Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5) * PLOT_SIZE).extend(0.0);
                        cmd.spawn((
                            SpriteBundle {
                                texture: assets.load("wheat_unit.png"),
                                transform: Transform::from_translation(pos + offset),
                                sprite: Sprite {
                                    anchor: bevy::sprite::Anchor::BottomCenter,
                                    ..default()
                                },
                                ..default()
                            },
                            Collider::ball(4.0),
                            Unit::new(
                                WHEAT_MOVE_SPEED,
                                WHEAT_AGGRO_RANGE,
                                WHEAT_CHASE_RANGE,
                                WHEAT_ATTACK_RANGE,
                                WHEAT_LEASH_RANGE,
                                WHEAT_ATTACK_SPEED,
                                WHEAT_DAMAGE,
                            ),
                            Health::new(WHEAT_HEALTH),
                            common.clone(),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    texture: assets.load("arrow.png"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -5.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        anchor: bevy::sprite::Anchor::TopCenter,
                                        color: Color::YELLOW,
                                        ..default()
                                    },
                                    visibility: Visibility::INVISIBLE,
                                    ..default()
                                },
                                SelectionIndicator,
                            ));
                            parent.spawn((
                                SpriteBundle {
                                    texture: assets.load("arrow.png"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -5.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        anchor: bevy::sprite::Anchor::TopCenter,
                                        ..default()
                                    },
                                    visibility: Visibility::INVISIBLE,
                                    ..default()
                                },
                                HoverIndicator,
                            ));
                            parent.spawn((
                                SpriteBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, -4.0, 0.1,
                                    )),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(2.0, 1.0)),
                                        color: Color::RED,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Bar {
                                    value: WHEAT_HEALTH,
                                    max: WHEAT_HEALTH,
                                    size: 10.0,
                                },
                                HealthBar,
                            ));
                        });
                    }
                }
            }
        }
    }

    fn enemy_spawn(mut q_enemy: Query<&mut Unit, Changed<Enemy>>) {
        for mut enemy in &mut q_enemy {
            enemy.command = Some(UnitCommand::AttackMove(Vec2::ZERO));
        }
    }

    fn process_command(
        mut q_unit: Query<(&mut Unit, &Selectable)>,
        mouse_buttons: Res<Input<MouseButton>>,
        keyboard: Res<Input<KeyCode>>,
        mouse_pos: Res<MousePosition>,
    ) {
        let mut command: Option<UnitCommand> = None;

        if mouse_buttons.just_pressed(MouseButton::Right) {
            command = Some(UnitCommand::Move(mouse_pos.truncate()));
        } else if keyboard.just_pressed(KeyCode::A) {
            command = Some(UnitCommand::AttackMove(mouse_pos.truncate()));
        }
        if command.is_some() {
            for (mut unit, selectable) in &mut q_unit {
                if selectable.selected {
                    unit.command = command.clone();
                }
            }
        }
    }

    fn crop_decay(
        time: Res<Time>,
        q_crop: Query<(Entity, &Crop)>,
        mut health_change: EventWriter<HealthChange>,
    ) {
        let delta = time.delta_seconds();
        for (target, crop) in &q_crop {
            let amount = -delta
                * match crop {
                    Crop::Carrot => CARROT_HEALTH / (CARROT_DECAY_TIME + CARROT_GROW_TIME),
                    Crop::Clover => CLOVER_HEALTH / (CLOVER_DECAY_TIME + CLOVER_GROW_TIME),
                    Crop::Wheat => WHEAT_HEALTH / (WHEAT_DECAY_TIME + WHEAT_GROW_TIME),
                };
            health_change.send(HealthChange { target, amount });
        }
    }

    fn flip_unit(mut q_sprite: Query<(&mut Sprite, &Velocity), With<Unit>>) {
        for (mut sprite, vel) in &mut q_sprite {
            if vel.linvel.x > 0.0 {
                sprite.flip_x = false;
            } else if vel.linvel.x < 0.1 {
                sprite.flip_x = true;
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::handle_harvest_event.run_in_state(GameState::InGame))
            .add_system(Self::process_unit_state.run_in_state(GameState::InGame))
            .add_system(Self::process_command.run_in_state(GameState::InGame))
            .add_system(Self::enemy_spawn.run_in_state(GameState::InGame))
            .add_system(Self::flip_unit.run_in_state(GameState::InGame))
            .add_system(Self::crop_decay.run_in_state(GameState::InGame))
            .add_system(Self::update_unit_state::<Friendly>.run_in_state(GameState::InGame))
            .add_system(Self::update_unit_state::<Enemy>.run_in_state(GameState::InGame));
    }
}
