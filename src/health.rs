use bevy::prelude::*;

use crate::utils::Bar;

#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Health {
            current: amount,
            max: amount,
        }
    }
    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct DamageFlash {
    amount: f32,
}

impl DamageFlash {
    pub fn new() -> Self {
        DamageFlash { amount: 0.0 }
    }
}

pub struct HealthChange {
    pub target: Entity,
    pub amount: f32,
}

#[derive(Component)]
pub struct HealthBar;

pub struct Plugin;

impl Plugin {
    fn update_health(
        mut cmd: Commands,
        mut q_health: Query<(Entity, &mut Health)>,
        mut event_reader: EventReader<HealthChange>,
    ) {
        for event in event_reader.iter() {
            if let Ok((entity, mut health)) = q_health.get_mut(event.target) {
                health.current += event.amount;

                if health.current > health.max {
                    health.current = health.max;
                } else if health.current <= 0.0 {
                    health.current = 0.0;
                    cmd.entity(entity).remove::<Health>().insert(Dead);
                }

                if event.amount < 0.0 {
                    cmd.entity(entity).insert(DamageFlash::new());
                }
            }
        }
    }

    fn update_healthbar(
        q_health: Query<(&Health, &Children)>,
        mut q_healthbar: Query<&mut Bar, With<HealthBar>>,
    ) {
        for (health, children) in &q_health {
            for child in children.iter() {
                if let Ok(mut bar) = q_healthbar.get_mut(*child) {
                    bar.value = health.current;
                }
            }
        }
    }

    fn init_damage_flash(
        mut q_flash: Query<AnyOf<(&mut Sprite, &mut TextureAtlasSprite)>, Added<DamageFlash>>,
    ) {
        for sprite in &mut q_flash {
            if let Some(mut sprite) = sprite.0 {
                sprite.color = Color::RED;
            } else if let Some(mut sprite) = sprite.1 {
                sprite.color = Color::RED;
            }
        }
    }

    fn update_damage_flash(
        mut cmd: Commands,
        mut q_flash: Query<(
            Entity,
            &mut DamageFlash,
            AnyOf<(&mut Sprite, &mut TextureAtlasSprite)>,
        )>,
        time: Res<Time>,
    ) {
        for (entity, mut flash, sprite) in &mut q_flash {
            flash.amount += time.delta_seconds() * 5.0;
            flash.amount = flash.amount.min(1.0);
            if let Some(mut sprite) = sprite.0 {
                sprite.color = Color::rgb(10.0 - 9.0 * flash.amount, flash.amount, flash.amount);
            } else if let Some(mut sprite) = sprite.1 {
                sprite.color = Color::rgb(10.0 - 9.0 * flash.amount, flash.amount, flash.amount);
            }

            if flash.amount >= 1.0 {
                cmd.entity(entity).remove::<DamageFlash>();
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthChange>()
            .add_system(Self::update_health)
            .add_system(Self::init_damage_flash)
            .add_system(Self::update_healthbar)
            .add_system(Self::update_damage_flash);
    }
}
