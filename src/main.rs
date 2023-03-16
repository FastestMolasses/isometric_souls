use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::time::Duration;

const CAMERA_SMOOTHING_FACTOR: f32 = 0.2;

#[derive(Component)]
struct Character {
    speed: f32,
    dash_duration: Timer,
    dashing: bool,
    last_move_direction: Vec2,
}

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

#[derive(Component, Default)]
struct AttackState {
    attack_chain: Vec<f32>,
    attack_timer: Timer,
    current_attack: usize,
    attack_pressed: bool,
}

#[derive(Component)]
struct Player;

// Input resources
#[derive(Resource, Default)]
struct InputState {
    move_direction: Vec2,
    attack: bool,
    dash: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<InputState>()
        .add_system(input_handling_system)
        .add_system(character_controller_system)
        .add_system(camera_follow_system)
        .add_system(player_death_system)
        .add_system(attack_handling_system)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("steel_armor.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 32, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    });
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
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

fn input_handling_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<InputState>,
    mut query: Query<&mut TextureAtlasSprite>,
) {
    for mut sprite in &mut query {
        sprite.index = 1;
    }

    let mut move_direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }

    input_state.move_direction = move_direction;
    input_state.attack = keyboard_input.pressed(KeyCode::Space);
    input_state.dash = keyboard_input.just_pressed(KeyCode::LShift);
}

fn character_controller_system(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(&mut Character, &mut Transform)>,
) {
    for (mut character, mut transform) in query.iter_mut() {
        // 8-directional movement
        if !character.dashing {
            if input_state.move_direction != Vec2::ZERO {
                let move_direction = input_state.move_direction.normalize();
                transform.translation +=
                    move_direction.extend(0.0) * character.speed * time.delta_seconds();
                character.last_move_direction = move_direction;
            }
        }

        // Dash
        if input_state.dash && !character.dashing {
            character.dashing = true;
            character.dash_duration.reset();
            println!("Start dashing!");
        }

        if character.dashing {
            character.dash_duration.tick(time.delta());
            if character.dash_duration.finished() {
                character.dashing = false;
                println!("End dashing!");
            } else {
                let move_direction = if input_state.move_direction != Vec2::ZERO {
                    input_state.move_direction.normalize()
                } else {
                    -character.last_move_direction // Dash in the opposite direction
                };

                let dash_speed = character.speed * 3.0;
                transform.translation +=
                    move_direction.extend(0.0) * dash_speed * time.delta_seconds();
            }
        }
    }
}

fn camera_follow_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for player_transform in query.iter_mut() {
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

fn deal_damage(mut health: &mut Health, damage: u32) {
    health.current = health.current.saturating_sub(damage);
}

fn player_death_system(query: Query<(Entity, &Health), With<Player>>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.current == 0 {
            println!("Player has died.");
            commands.entity(entity).despawn();
        }
    }
}

fn attack_handling_system(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(&mut AttackState, &Character)>,
) {
    for (mut attack_state, character) in query.iter_mut() {
        if character.dashing {
            continue;
        }

        attack_state.attack_timer.tick(time.delta());

        if input_state.attack && !attack_state.attack_pressed {
            attack_state.attack_pressed = true;

            if attack_state.current_attack == 0 || attack_state.attack_timer.finished() {
                // Start the first attack or chain the next attack
                attack_state.current_attack =
                    (attack_state.current_attack) % attack_state.attack_chain.len() + 1;

                let current_attack_duration =
                    attack_state.attack_chain[attack_state.current_attack - 1];
                attack_state
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(current_attack_duration));
                attack_state.attack_timer.reset();

                println!("Attack {}!", attack_state.current_attack);
            }
        } else if !input_state.attack {
            attack_state.attack_pressed = false;
            if attack_state.attack_timer.finished() {
                // If the attack button isn't pressed and the timer has finished, reset the current attack
                attack_state.current_attack = 0;
            }
        }
    }
}
