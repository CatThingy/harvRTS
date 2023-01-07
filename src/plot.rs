use std::f32::consts::TAU;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::{
        CARROT_DECAY_TIME, CARROT_GROW_TIME, PLOT_CIRCLE_BUTTON_RADIUS, PLOT_CIRCLE_RADIUS,
        PLOT_COLLISION_GROUP, PLOT_SIZE,
    },
    utils::MousePosition,
    GameState,
};

#[derive(Component, Default, Debug)]
pub enum Plot {
    #[default]
    Empty,
    Growing(Crop, f32),
    Ready(Crop, f32),
}

#[derive(Component)]
pub struct PlotOverlay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Crop {
    Carrot,
}

#[derive(Component)]
pub struct PlotCircle {
    target: Entity,
}

#[derive(Resource, Deref, Default)]
pub struct ActivePlotCircle(Option<Entity>);

#[derive(Component, Debug)]
pub struct PlotCircleButton {
    action: PlotAction,
}

#[derive(Debug, Clone)]
enum PlotAction {
    Plant(Crop),
    Harvest,
    Cancel,
}

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        for offset in [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, -1.0),
        ] {
            cmd.spawn((
                SpriteBundle {
                    texture: assets.load("plot.png"),
                    sprite: Sprite {
                        custom_size: Some(PLOT_SIZE),
                        ..default()
                    },

                    transform: Transform::from_translation(Vec3::new(
                        offset.x * PLOT_SIZE.x,
                        offset.y * PLOT_SIZE.y,
                        0.0,
                    )),
                    ..default()
                },
                Plot::default(),
                Collider::cuboid(PLOT_SIZE.x / 2.0, PLOT_SIZE.y / 2.0),
                Sensor,
                CollisionGroups {
                    memberships: PLOT_COLLISION_GROUP,
                    filters: Group::NONE,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: assets.load("empty.png"),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                        ..default()
                    },
                    PlotOverlay,
                ));
            });
        }
    }

    fn plot_click(
        mut cmd: Commands,
        assets: Res<AssetServer>,
        mouse_pos: Res<MousePosition>,
        mouse_button: Res<Input<MouseButton>>,
        rapier_ctx: Res<RapierContext>,
        mut plot_circle: ResMut<ActivePlotCircle>,
        q_plot: Query<(Entity, &GlobalTransform), With<Plot>>,
    ) {
        if mouse_button.just_pressed(MouseButton::Left) && plot_circle.is_none() {
            let mut plot = None;
            rapier_ctx.intersections_with_point(mouse_pos.0.truncate(), QueryFilter::new(), |e| {
                if let Ok((entity, transform)) = q_plot.get(e) {
                    plot = Some((entity, transform.translation()));
                    return true;
                }
                false
            });
            let Some((entity, mut pos)) = plot else { return; };

            pos.z = 1.0;

            let new_plot_circle = cmd
                .spawn((
                    SpriteBundle {
                        texture: assets.load("plot_circle.png"),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    PlotCircle { target: entity },
                ))
                .with_children(|v| {
                    //  carrot
                    v.spawn((
                        SpriteBundle {
                            texture: assets.load("plant_carrot.png"),
                            transform: Transform::from_translation(Vec3 {
                                x: f32::cos(TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                y: f32::sin(TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                z: 0.1,
                            }),
                            ..default()
                        },
                        PlotCircleButton {
                            action: PlotAction::Plant(Crop::Carrot),
                        },
                    ));
                    // v.spawn((
                    //     SpriteBundle {
                    //         texture: assets.load("cancel.png"),
                    //         transform: Transform::from_translation(Vec3 {
                    //             x: f32::cos(3.0 * TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                    //             y: f32::sin(3.0 * TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                    //             z: 0.1,
                    //         }),
                    //         ..default()
                    //     },
                    //     PlotCircleButton {
                    //         action: PlotAction::Cancel,
                    //     },
                    // ));
                })
                .id();

            plot_circle.0 = Some(new_plot_circle);
        }
    }

    fn remove_plot_circle(
        mut cmd: Commands,
        mouse_pos: Res<MousePosition>,
        mouse_button: Res<Input<MouseButton>>,
        mut plot_circle: ResMut<ActivePlotCircle>,
        q_plot_circle: Query<(Entity, &GlobalTransform), With<PlotCircle>>,
    ) {
        if mouse_button.just_pressed(MouseButton::Left) {
            if let Some((entity, transform)) = plot_circle.map(|v| q_plot_circle.get(v).unwrap()) {
                let dist = transform
                    .translation()
                    .truncate()
                    .distance(mouse_pos.truncate());

                if dist > PLOT_CIRCLE_RADIUS {
                    cmd.entity(entity).despawn_recursive();
                    plot_circle.0 = None;
                    return;
                }

                let mut relative_mouse_pos = mouse_pos.0 - transform.translation();
                relative_mouse_pos.z = 0.0;
            }
        }
    }

    fn plot_button_click(
        mut cmd: Commands,
        mut plot_event: EventWriter<PlotAction>,
        mouse_pos: Res<MousePosition>,
        mouse_button: Res<Input<MouseButton>>,
        q_plot_circle_button: Query<(Entity, &GlobalTransform, &PlotCircleButton)>,
    ) {
        if mouse_button.just_pressed(MouseButton::Left) {
            for (entity, transform, button) in &q_plot_circle_button {
                let dist = transform
                    .translation()
                    .truncate()
                    .distance(mouse_pos.truncate());

                if dist <= PLOT_CIRCLE_BUTTON_RADIUS {
                    cmd.entity(entity).remove::<PlotCircleButton>();
                    plot_event.send(button.action.clone());
                    return;
                }
            }
        }
    }

    fn handle_plot_event(
        mut cmd: Commands,
        mut plot_events: ResMut<Events<PlotAction>>,
        mut plot_circle: ResMut<ActivePlotCircle>,
        q_plot_circle: Query<&PlotCircle>,
        mut q_plots: Query<&mut Plot>,
    ) {
        for event in plot_events.drain() {
            let plot_circle = plot_circle.0.take().unwrap();
            cmd.entity(plot_circle).despawn_recursive();

            let target = q_plot_circle.get(plot_circle).unwrap().target;

            let mut plot = q_plots.get_mut(target).unwrap();

            match &*plot {
                Plot::Empty => match event {
                    PlotAction::Plant(crop) => *plot = Plot::Growing(crop, 0.0),
                    PlotAction::Harvest | PlotAction::Cancel => {
                        error!("can't harvest or cancel an empty plot")
                    }
                },
                Plot::Growing(_, _) => match event {
                    PlotAction::Plant(_) | PlotAction::Harvest => {
                        error!("can't plant in or harvest a growing plot")
                    }
                    PlotAction::Cancel => *plot = Plot::Empty,
                },
                Plot::Ready(_, _) => match event {
                    PlotAction::Plant(_) => error!("can't plant in a grown plot"),
                    PlotAction::Harvest | PlotAction::Cancel => *plot = Plot::Empty,
                },
            }
        }
    }

    fn update_plot_overlay(
        assets: Res<AssetServer>,
        q_plot: Query<(&Plot, &Children), Changed<Plot>>,
        mut q_overlay: Query<&mut Handle<Image>, With<PlotOverlay>>,
    ) {
        for (plot, children) in &q_plot {
            for child in children.iter() {
                if let Ok(mut sprite) = q_overlay.get_mut(*child) {
                    *sprite = match plot {
                        Plot::Empty => assets.load("empty.png"),
                        Plot::Growing(_, _) => assets.load("growing.png"),
                        Plot::Ready(_, _) => assets.load("grown.png"),
                    };
                }
            }
        }
    }

    fn update_plot(
        time: Res<Time>,
        active_plot_circle: Res<ActivePlotCircle>,
        mut plots: Query<&mut Plot>,
    ) {
        let delta = time.delta_seconds();
        for mut plot in &mut plots {
            match &mut *plot {
                Plot::Empty => {}
                Plot::Growing(crop, ref mut t) => {
                    match crop {
                        Crop::Carrot => *t += delta / CARROT_GROW_TIME,
                    }
                    if *t >= 1.0 && active_plot_circle.0.is_none() {
                        *plot = Plot::Ready(crop.clone(), 0.0)
                    }
                }
                Plot::Ready(crop, ref mut t) => {
                    match crop {
                        Crop::Carrot => *t += delta / CARROT_DECAY_TIME,
                    }
                    if *t >= 1.0 && active_plot_circle.0.is_none() {
                        *plot = Plot::Empty;
                    }
                }
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivePlotCircle>()
            .init_resource::<Events<PlotAction>>()
            .add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::remove_plot_circle.before("plot_click"))
            .add_system(Self::plot_click.label("plot_click"))
            .add_system(Self::plot_button_click.after("plot_click"))
            .add_system(Self::handle_plot_event)
            .add_system(Self::update_plot_overlay)
            .add_system(Self::update_plot);
    }
}
