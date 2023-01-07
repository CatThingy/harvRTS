use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::{SELECTION_COLLISION_GROUP, UNIT_COLLISION_GROUP},
    utils::MousePosition,
    GameState,
};

#[derive(Default, Component)]
pub struct SelectionBox {
    start: Vec2,
    end: Vec2,
    selecting: bool,
}

pub struct ConfirmSelectionEvent;

#[derive(Default, Clone, Component)]
pub struct Selectable {
    pub hovered: bool,
    pub selected: bool,
}

#[derive(Component)]
pub struct SelectionIndicator;

#[derive(Component)]
pub struct HoverIndicator;

pub struct Plugin;
impl Plugin {
    fn box_select(
        mut q_box: Query<(
            &mut Sprite,
            &mut SelectionBox,
            &mut Transform,
            &mut Visibility,
        )>,
        mouse_buttons: Res<Input<MouseButton>>,
        mouse_pos: Res<MousePosition>,
        mut ev_confirm: EventWriter<ConfirmSelectionEvent>,
    ) {
        let (mut box_sprite, mut box_data, mut box_transform, mut box_visibility) =
            q_box.single_mut();
        if mouse_buttons.just_pressed(MouseButton::Left) {
            box_data.start = mouse_pos.truncate();
            box_data.end = mouse_pos.truncate();
            box_data.selecting = true;
            box_visibility.is_visible = true;
        } else if mouse_buttons.pressed(MouseButton::Left) && box_visibility.is_visible {
            box_data.end = mouse_pos.truncate();
        } else if mouse_buttons.just_released(MouseButton::Left) && box_visibility.is_visible {
            box_data.selecting = false;
            box_visibility.is_visible = false;

            ev_confirm.send(ConfirmSelectionEvent);
        }
        // Handle hovering
        else {
            box_data.start = mouse_pos.truncate();
            box_data.end = mouse_pos.truncate();
        }

        let box_size = box_data.start - box_data.end;

        box_sprite.custom_size = Some(box_size);
        box_transform.translation = (box_data.start - box_size / 2.0).extend(99.0);
    }

    fn update(
        q_box: Query<&SelectionBox>,
        rapier_context: Res<RapierContext>,
        mut q_selectable: Query<(Entity, &mut Selectable, Option<&Children>)>,
        mut q_selection_indicator: Query<
            &mut Visibility,
            (With<SelectionIndicator>, Without<HoverIndicator>),
        >,
        mut q_hover_indicator: Query<
            &mut Visibility,
            (With<HoverIndicator>, Without<SelectionIndicator>),
        >,
        mut ev_confirm: ResMut<Events<ConfirmSelectionEvent>>,
        keyboard: Res<Input<KeyCode>>,
    ) {
        let selection_confirmed = ev_confirm.drain().next().is_some();
        let box_data = q_box.single();

        let box_extents = (box_data.start - box_data.end).abs();

        // Some(false) => single select, no selected
        // Some(true) => single select, selected
        // None => not single select
        let mut select_single = if box_extents.x < 5.0 && box_extents.y < 5.0 {
            Some(false)
        } else {
            None
        };

        let mut selected = HashSet::<Entity>::new();

        rapier_context.intersections_with_shape(
            (box_data.start + box_data.end) / 2.0,
            0.0,
            &Collider::cuboid(box_extents.x / 2.0, box_extents.y / 2.0),
            QueryFilter::new().groups(InteractionGroups {
                memberships: UNIT_COLLISION_GROUP.bits().into(),
                filter: SELECTION_COLLISION_GROUP.bits().into(),
            }),
            |e| {
                if q_selectable.contains(e) {
                    selected.insert(e);
                };

                true
            },
        );

        for (entity, mut selectable_data, selectable_children) in &mut q_selectable {
            let mut updated = select_single.unwrap_or(false);

            if !updated && selected.contains(&entity) {
                if selection_confirmed {
                    // Invert selection if shift held
                    if keyboard.pressed(KeyCode::RShift) || keyboard.pressed(KeyCode::LShift) {
                        selectable_data.selected = !selectable_data.selected;
                    } else {
                        selectable_data.selected = true;
                    }
                }
                selectable_data.hovered = true;
                updated = true;
            } else if selection_confirmed {
                selectable_data.hovered = false;
                // Only reset selection if shift not held
                if !(keyboard.pressed(KeyCode::RShift) || keyboard.pressed(KeyCode::LShift)) {
                    selectable_data.selected = false;
                }
            } else {
                selectable_data.hovered = false;
            }
            // Display indicators for hover and select
            if let Some(selectable_children) = selectable_children {
                for &child in selectable_children.iter() {
                    if let Ok(mut selection_indicator) = q_selection_indicator.get_mut(child) {
                        selection_indicator.is_visible = selectable_data.selected;
                    }
                    if let Ok(mut hover_indicator) = q_hover_indicator.get_mut(child) {
                        hover_indicator.is_visible = selectable_data.hovered;
                    }
                }
            }

            if updated {
                select_single = select_single.map(|_| true);
            }
        }
    }

    fn init(mut commands: Commands) {
        commands.spawn((
            SelectionBox::default(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                        alpha: 0.2,
                    },
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 99.0)),
                ..default()
            },
        ));
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<ConfirmSelectionEvent>>()
            .add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::update.run_in_state(GameState::InGame))
            .add_system(Self::box_select.run_in_state(GameState::InGame));
    }
}
