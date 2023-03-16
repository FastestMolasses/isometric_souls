mod camera;
mod health;
mod player;
mod util;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use health::Health;
use player::{Character, DirectionAtlasHandles, Player, AttackState};
use util::Direction;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(health::HealthPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut direction_atlas_handles: ResMut<DirectionAtlasHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load the directional player textures
    let texture_handles = [
        ("HeroEast.png", Direction::East),
        ("HeroNorth.png", Direction::North),
        ("HeroNorthEast.png", Direction::NorthEast),
        ("HeroSouth.png", Direction::South),
        ("HeroSouthEast.png", Direction::SouthEast),
    ];
    for (file_name, direction) in &texture_handles {
        let texture_handle = asset_server.load(*file_name);
        let atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 12, None, None);
        let atlas_handle = texture_atlases.add(atlas);
        direction_atlas_handles.0.insert(*direction, atlas_handle);
    }

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    });
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: direction_atlas_handles
                .0
                .get(&Direction::East)
                .unwrap()
                .clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        Character {
            speed: 300.0,
            dash_duration: Timer::from_seconds(0.25, TimerMode::Repeating),
            dashing: false,
            last_move_direction: Vec2::new(1.0, 0.0),
        },
        AttackState {
            attack_chain: vec![0.5, 0.4, 0.6], // Attack durations for each attack in the chain
            attack_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            ..Default::default()
        },
        Health {
            current: 100,
            max: 100,
        },
        Player,
    ));

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}
