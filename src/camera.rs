use crate::player::Player;
use bevy::prelude::*;

const CAMERA_SMOOTHING_FACTOR: f32 = 0.2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_follow_system);
    }
}

fn camera_follow_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for player_transform in player_query.iter_mut() {
        for mut camera_transform in camera_query.iter_mut() {
            // Use lerp for camera smoothing
            camera_transform.translation.x = camera_transform.translation.x
                + (player_transform.translation.x - camera_transform.translation.x)
                    * CAMERA_SMOOTHING_FACTOR;
            camera_transform.translation.y = camera_transform.translation.y
                + (player_transform.translation.y - camera_transform.translation.y)
                    * CAMERA_SMOOTHING_FACTOR;
        }
    }
}
