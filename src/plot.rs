use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::{
        CARROT_COMPOST, CARROT_COST, CARROT_DECAY_TIME, CARROT_GROW_TIME, CLOVER_COMPOST,
        CLOVER_COST, CLOVER_DECAY_TIME, CLOVER_GROW_TIME, PLOT_CIRCLE_BUTTON_RADIUS,
        PLOT_CIRCLE_RADIUS, PLOT_COLLISION_GROUP, PLOT_SIZE, PLOT_UNLOCK_COST, WHEAT_COMPOST,
        WHEAT_COST, WHEAT_DECAY_TIME, WHEAT_GROW_TIME,
    },
    game::Compost,
    selection::Selectable,
    utils::MousePosition,
    GameState,
};

#[derive(Component, Default, Debug)]
pub enum Plot {
    #[default]
    Locked,
    Empty,
    Growing(Crop, f32),
    Ready(Crop, f32),
}

#[derive(Component)]
pub struct PlotOverlay;

#[derive(Debug, Clone, PartialEq, Component)]
pub enum Crop {
    Carrot,
    Clover,
    Wheat,
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
    Harvest(Crop),
    Cancel,
    Compost(Crop),
    Unlock,
}

pub struct HarvestEvent {
    pub crop: Crop,
    pub pos: Vec3,
}

#[derive(Component)]
pub struct CompostDisplay;

#[derive(Component)]
pub struct CompostDisplayText;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        for offset in [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
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
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            -PLOT_SIZE.y / 2.0,
                            0.0001,
                        )),
                        sprite: Sprite {
                            anchor: bevy::sprite::Anchor::BottomCenter,
                            ..default()
                        },
                        ..default()
                    },
                    PlotOverlay,
                ));
            });
        }
        cmd.spawn((
            SpriteBundle {
                texture: assets.load("plot.png"),
                sprite: Sprite {
                    custom_size: Some(PLOT_SIZE),
                    ..default()
                },

                transform: Transform::from_translation(Vec3::new(0.0, -PLOT_SIZE.y, 0.0)),
                ..default()
            },
            Plot::Empty,
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
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        -PLOT_SIZE.y / 2.0,
                        0.0001,
                    )),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::BottomCenter,
                        ..default()
                    },
                    ..default()
                },
                PlotOverlay,
            ));
        });
    }

    fn plot_click(
        mut cmd: Commands,
        assets: Res<AssetServer>,
        mouse_pos: Res<MousePosition>,
        mouse_button: Res<Input<MouseButton>>,
        rapier_ctx: Res<RapierContext>,
        mut plot_circle: ResMut<ActivePlotCircle>,
        q_plot: Query<(Entity, &Plot, &GlobalTransform)>,
        q_selectable: Query<&Selectable>,
    ) {
        for selectable in &q_selectable {
            if selectable.hovered {
                return;
            }
        }
        if mouse_button.just_released(MouseButton::Left) && plot_circle.is_none() {
            let mut plot = None;
            rapier_ctx.intersections_with_point(mouse_pos.0.truncate(), QueryFilter::new(), |e| {
                if let Ok((entity, plot_data, transform)) = q_plot.get(e) {
                    plot = Some((entity, plot_data, transform.translation()));
                    return true;
                }
                false
            });
            let Some((entity, plot, mut pos)) = plot else { return; };

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
                    v.spawn((SpatialBundle::INVISIBLE_IDENTITY, CompostDisplay))
                        .with_children(|v| {
                            v.spawn((
                                Text2dBundle {
                                    text: Text::from_section(
                                        "00",
                                        TextStyle {
                                            font: assets.load("fonts/ModeSeven.ttf"),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    transform: Transform {
                                        scale: Vec3::splat(0.25),
                                        translation: Vec3::new(0.0, 2.0, 0.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                CompostDisplayText,
                            ));
                            v.spawn(SpriteBundle {
                                texture: assets.load("compost.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(4.0)),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(-2.0, 0.0, 0.0)),
                                ..default()
                            });
                        });
                    match plot {
                        Plot::Locked => {
                            v.spawn((
                                SpriteBundle {
                                    texture: assets.load("compost.png"),
                                    transform: Transform::from_translation(Vec3 {
                                        x: f32::cos(TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        y: f32::sin(TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        z: 0.1,
                                    }),
                                    ..default()
                                },
                                PlotCircleButton {
                                    action: PlotAction::Unlock,
                                },
                            ));
                        }
                        Plot::Empty => {
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
                            // clover

                            v.spawn((
                                SpriteBundle {
                                    texture: assets.load("plant_clover.png"),
                                    transform: Transform::from_translation(Vec3 {
                                        x: f32::cos(7.0 * TAU / 12.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        y: f32::sin(7.0 * TAU / 12.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        z: 0.1,
                                    }),
                                    ..default()
                                },
                                PlotCircleButton {
                                    action: PlotAction::Plant(Crop::Clover),
                                },
                            ));
                            // wheat
                            v.spawn((
                                SpriteBundle {
                                    texture: assets.load("plant_wheat.png"),
                                    transform: Transform::from_translation(Vec3 {
                                        x: f32::cos(11.0 * TAU / 12.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        y: f32::sin(11.0 * TAU / 12.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        z: 0.1,
                                    }),
                                    ..default()
                                },
                                PlotCircleButton {
                                    action: PlotAction::Plant(Crop::Wheat),
                                },
                            ));
                        }
                        Plot::Growing(_, _) => {
                            v.spawn((
                                SpriteBundle {
                                    texture: assets.load("cancel.png"),
                                    transform: Transform::from_translation(Vec3 {
                                        x: f32::cos(3.0 * TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        y: f32::sin(3.0 * TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        z: 0.1,
                                    }),
                                    ..default()
                                },
                                PlotCircleButton {
                                    action: PlotAction::Cancel,
                                },
                            ));
                        }
                        Plot::Ready(crop, _) => {
                            v.spawn((
                                SpriteBundle {
                                    texture: assets.load("compost.png"),
                                    transform: Transform::from_translation(Vec3 {
                                        x: f32::cos(3.0 * TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        y: f32::sin(3.0 * TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        z: 0.1,
                                    }),
                                    ..default()
                                },
                                PlotCircleButton {
                                    action: PlotAction::Compost(crop.clone()),
                                },
                            ));
                            v.spawn((
                                SpriteBundle {
                                    texture: assets.load("harvest.png"),
                                    transform: Transform::from_translation(Vec3 {
                                        x: f32::cos(TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        y: f32::sin(TAU / 4.0) * PLOT_CIRCLE_RADIUS * 0.75,
                                        z: 0.1,
                                    }),
                                    ..default()
                                },
                                PlotCircleButton {
                                    action: PlotAction::Harvest(crop.clone()),
                                },
                            ));
                        }
                    }
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
        if mouse_button.just_released(MouseButton::Left) {
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

    fn plot_button_hover(
        q_plot_circle_button: Query<(&GlobalTransform, &PlotCircleButton)>,
        mouse_pos: Res<MousePosition>,
        mut q_compost_display: Query<&mut Visibility, With<CompostDisplay>>,
        mut q_compost_display_text: Query<&mut Text, With<CompostDisplayText>>,
    ) {
        for (transform, button) in &q_plot_circle_button {
            let dist = transform
                .translation()
                .truncate()
                .distance(mouse_pos.truncate());

            if dist <= PLOT_CIRCLE_BUTTON_RADIUS {
                match &button.action {
                    PlotAction::Plant(crop) => {
                        q_compost_display.single_mut().is_visible = true;
                        let mut text = q_compost_display_text.single_mut();
                        let number = match crop {
                            Crop::Carrot => CARROT_COST,
                            Crop::Clover => CLOVER_COST,
                            Crop::Wheat => WHEAT_COST,
                        };

                        text.sections[0].value = format!("{number}");
                        text.sections[0].style.color = Color::RED;
                    }
                    PlotAction::Compost(crop) => {
                        q_compost_display.single_mut().is_visible = true;
                        let mut text = q_compost_display_text.single_mut();
                        let number = match crop {
                            Crop::Carrot => CARROT_COMPOST,
                            Crop::Clover => CLOVER_COMPOST,
                            Crop::Wheat => WHEAT_COMPOST,
                        };

                        text.sections[0].value = format!("{number}");
                        text.sections[0].style.color = Color::GREEN;
                    }
                    PlotAction::Unlock => {
                        q_compost_display.single_mut().is_visible = true;
                        let mut text = q_compost_display_text.single_mut();

                        text.sections[0].value = format!("{PLOT_UNLOCK_COST}");
                        text.sections[0].style.color = Color::RED;
                    }
                    _ => {}
                }
                return;
            }
        }

        if let Ok(mut display) = q_compost_display.get_single_mut() {
            display.is_visible = false;
        }
    }

    fn handle_plot_event(
        mut cmd: Commands,
        mut plot_events: ResMut<Events<PlotAction>>,
        mut harvest_events: EventWriter<HarvestEvent>,
        mut compost: ResMut<Compost>,
        mut plot_circle: ResMut<ActivePlotCircle>,
        q_plot_circle: Query<&PlotCircle>,
        mut q_plots: Query<(&mut Plot, &GlobalTransform)>,
    ) {
        for event in plot_events.drain() {
            let plot_circle = plot_circle.0.take().unwrap();
            cmd.entity(plot_circle).despawn_recursive();

            let target = q_plot_circle.get(plot_circle).unwrap().target;

            let (mut plot, transform) = q_plots.get_mut(target).unwrap();

            match &*plot {
                Plot::Locked => match event {
                    PlotAction::Unlock => {
                        if compost.0 >= PLOT_UNLOCK_COST {
                            *plot = Plot::Empty;
                            compost.0 -= PLOT_UNLOCK_COST;
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                },
                Plot::Empty => match event {
                    PlotAction::Plant(crop) => {
                        let cost = match crop {
                            Crop::Carrot => CARROT_COST,
                            Crop::Clover => CLOVER_COST,
                            Crop::Wheat => WHEAT_COST,
                        };

                        if compost.0 >= cost {
                            *plot = Plot::Growing(crop, 0.0);
                            compost.0 -= cost;
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                },
                Plot::Growing(_, _) => match event {
                    PlotAction::Cancel => *plot = Plot::Empty,
                    _ => {
                        unreachable!();
                    }
                },
                Plot::Ready(crop, _) => match event {
                    PlotAction::Harvest(_) => {
                        harvest_events.send(HarvestEvent {
                            crop: crop.clone(),
                            pos: transform.translation(),
                        });
                        *plot = Plot::Empty;
                    }
                    PlotAction::Compost(_) => {
                        compost.0 += match crop {
                            Crop::Carrot => CARROT_COMPOST,
                            Crop::Clover => CLOVER_COMPOST,
                            Crop::Wheat => WHEAT_COMPOST,
                        };
                        *plot = Plot::Empty;
                    }
                    _ => {
                        unreachable!();
                    }
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
                        Plot::Locked => assets.load("rocks.png"),
                        Plot::Empty => assets.load("empty.png"),
                        Plot::Growing(crop, _) => match crop {
                            Crop::Carrot => assets.load("carrot_growing.png"),
                            Crop::Clover => assets.load("clover_growing.png"),
                            Crop::Wheat => assets.load("wheat_growing.png"),
                        },
                        Plot::Ready(crop, _) => match crop {
                            Crop::Carrot => assets.load("carrot_grown.png"),
                            Crop::Clover => assets.load("clover_grown.png"),
                            Crop::Wheat => assets.load("wheat_grown.png"),
                        },
                    };
                }
            }
        }
    }

    fn update_plot(
        time: Res<Time>,
        active_plot_circle: Res<ActivePlotCircle>,
        mut compost: ResMut<Compost>,
        mut plots: Query<&mut Plot>,
    ) {
        let delta = time.delta_seconds();
        for mut plot in &mut plots {
            match &mut *plot {
                Plot::Empty | Plot::Locked => {}
                Plot::Growing(crop, ref mut t) => {
                    match crop {
                        Crop::Carrot => *t += delta / CARROT_GROW_TIME,
                        Crop::Clover => *t += delta / CLOVER_GROW_TIME,
                        Crop::Wheat => *t += delta / WHEAT_GROW_TIME,
                    }
                    if *t >= 1.0 && active_plot_circle.0.is_none() {
                        *plot = Plot::Ready(crop.clone(), 0.0)
                    }
                }
                Plot::Ready(crop, ref mut t) => {
                    match crop {
                        Crop::Carrot => *t += delta / CARROT_DECAY_TIME,
                        Crop::Clover => *t += delta / CLOVER_DECAY_TIME,
                        Crop::Wheat => *t += delta / WHEAT_DECAY_TIME,
                    }
                    if *t >= 1.0 && active_plot_circle.0.is_none() {
                        compost.0 += match crop {
                            Crop::Carrot => CARROT_COMPOST,
                            Crop::Clover => CLOVER_COMPOST,
                            Crop::Wheat => WHEAT_COMPOST,
                        } / 2;
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
            .init_resource::<Events<HarvestEvent>>()
            .add_enter_system(GameState::InGame, Self::init)
            .add_system(
                Self::remove_plot_circle
                    .run_in_state(GameState::InGame)
                    .before("plot_click"),
            )
            .add_system(
                Self::plot_click
                    .run_in_state(GameState::InGame)
                    .label("plot_click"),
            )
            .add_system(
                Self::plot_button_click
                    .run_in_state(GameState::InGame)
                    .after("plot_click"),
            )
            .add_system(
                Self::plot_button_hover
                    .run_in_state(GameState::InGame)
                    .after("plot_click"),
            )
            .add_system(Self::handle_plot_event.run_in_state(GameState::InGame))
            .add_system(Self::update_plot_overlay.run_in_state(GameState::InGame))
            .add_system(Self::update_plot.run_in_state(GameState::InGame));
    }
}
