use bevy::prelude::*;
use crate::player::Player;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_death_system);
    }
}

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

pub fn deal_damage(mut health: &mut Health, damage: u32) {
    health.current = health.current.saturating_sub(damage);
}

pub fn player_death_system(query: Query<(Entity, &Health), With<Player>>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.current == 0 {
            println!("Player has died.");
            commands.entity(entity).despawn();
        }
    }
}
