use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::consts::*;
use crate::game_menu::CompostText;
use crate::game_menu::GameMenu;
use crate::game_menu::GameTimer;
use crate::health::Health;
use crate::health::HealthBar;
use crate::unit::Enemy;
use crate::unit::Unit;
use crate::utils::Bar;
use crate::GameState;
use crate::MainCamera;

#[derive(Component)]
pub struct Rose;

#[derive(Resource)]
pub struct Spawner {
    pub enemy: Timer,
    pub total: Duration,
}

#[derive(Resource)]
pub struct Compost(pub u32);

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        cmd.spawn((
            SpriteBundle {
                texture: assets.load("rose.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..default()
                },

                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
                ..default()
            },
            Health::new(100.0),
            Rose,
            RigidBody::Fixed,
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Collider::ball(4.0),
            CollisionGroups {
                memberships: UNIT_COLLISION_GROUP | FRIENDLY_COLLISION_GROUP,
                filters: UNIT_COLLISION_GROUP,
            },
        ));
    }

    fn enemy_spawning(
        mut cmd: Commands,
        assets: Res<AssetServer>,
        camera: Query<&Camera, With<MainCamera>>,
        mut spawner: ResMut<Spawner>,
        time: Res<Time>,
    ) {
        let camera = camera.single();

        spawner.total += time.delta();

        let tick_multiplier = spawner.total.as_secs_f32() / (2.0 * 60.0);

        spawner
            .enemy
            .tick(time.delta().mul_f32(1.0 + tick_multiplier));

        let rng = fastrand::Rng::default();

        if spawner.enemy.finished() {
            let viewport_size = camera.logical_viewport_size().unwrap();
            let spawn_area = viewport_size / 4.0 - viewport_size / 2.0;

            let x_sign = if rng.bool() { -1.0 } else { 1.0 };
            let y_sign = if rng.bool() { -1.0 } else { 1.0 };

            cmd.spawn((
                SpriteBundle {
                    texture: assets.load("enemy.png"),
                    transform: Transform::from_translation(Vec3::new(
                        (rng.f32() * 0.25 + 1.0) * spawn_area.x * x_sign,
                        (rng.f32() * 0.25 + 1.0) * spawn_area.y * y_sign,
                        0.1,
                    )),
                    ..default()
                },
                RigidBody::Dynamic,
                Velocity::default(),
                Collider::ball(4.0),
                LockedAxes::ROTATION_LOCKED_Z,
                CollisionGroups {
                    memberships: UNIT_COLLISION_GROUP | ENEMY_COLLISION_GROUP,
                    filters: UNIT_COLLISION_GROUP,
                },
                Damping {
                    linear_damping: 20.0,
                    angular_damping: 0.0,
                },
                Unit::new(
                    ENEMY_MOVE_SPEED,
                    ENEMY_AGGRO_RANGE,
                    ENEMY_CHASE_RANGE,
                    ENEMY_ATTACK_RANGE,
                    ENEMY_LEASH_RANGE,
                    ENEMY_ATTACK_SPEED,
                    ENEMY_DAMAGE,
                ),
                Enemy,
                Health::new(ENEMY_HEALTH),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(0.0, -4.0, 0.1)),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(2.0, 1.0)),
                            color: Color::RED,
                            ..default()
                        },
                        ..default()
                    },
                    Bar {
                        value: 5.0,
                        max: 5.0,
                        size: 10.0,
                    },
                    HealthBar,
                ));
            });
        }
    }

    fn end_game(
        mut time: ResMut<Time>,
        q_rose: Query<&Rose>,
        mut q_panel: Query<&mut Style, With<GameMenu>>,
    ) {
        if q_rose.is_empty() {
            let mut panel = q_panel.single_mut();
            panel.display = Display::Flex;
            time.set_relative_speed(0.0);
        }
    }

    fn update_timer(mut q_timer: Query<&mut Text, With<GameTimer>>, spawner: Res<Spawner>) {
        let mut timer = q_timer.single_mut();

        let seconds = spawner.total.as_secs();

        timer.sections[0].value = format!("{:<02}:{:<02}", seconds / 60, seconds % 60);
    }

    fn update_compost(compost: Res<Compost>, mut q_text: Query<&mut Text, With<CompostText>>) {
        let mut text = q_text.single_mut();
        text.sections[0].value = format!("{}", compost.0);
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Spawner {
            enemy: Timer::from_seconds(7.0, TimerMode::Repeating),
            total: Duration::default(),
        })
        .insert_resource(Compost(100))
        .add_enter_system(GameState::InGame, Self::init)
        .add_system(Self::end_game.run_in_state(GameState::InGame))
        .add_system(Self::update_timer.run_in_state(GameState::InGame))
        .add_system(Self::update_compost.run_in_state(GameState::InGame))
        .add_system(Self::enemy_spawning.run_in_state(GameState::InGame));
    }
}
