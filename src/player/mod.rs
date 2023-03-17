pub mod animation;
pub mod input;

use bevy::{app::{App, Plugin}, ecs::component::Component};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<input::InputState>()
            .init_resource::<animation::DirectionAtlasHandles>()
            .add_system(input::character_controller_system)
            .add_system(input::input_handling_system);
    }
}
