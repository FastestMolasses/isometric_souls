use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub fn vec2_to_direction(vector: Vec2) -> Direction {
    match vector.try_normalize() {
        Some(normalized_vector) => {
            let x = normalized_vector.x;
            let y = normalized_vector.y;

            if y >= 0.5 && x.abs() <= 0.5 {
                Direction::North
            } else if y <= -0.5 && x.abs() <= 0.5 {
                Direction::South
            } else if x >= 0.5 && y.abs() <= 0.5 {
                Direction::East
            } else if x <= -0.5 && y.abs() <= 0.5 {
                Direction::West
            } else if x >= 0.5 && y >= 0.5 {
                Direction::NorthEast
            } else if x <= -0.5 && y >= 0.5 {
                Direction::NorthWest
            } else if x >= 0.5 && y <= -0.5 {
                Direction::SouthEast
            } else if x <= -0.5 && y <= -0.5 {
                Direction::SouthWest
            } else {
                Direction::East
            }
        }
        None => Direction::East,
    }
}

/// NOTE: I expect to remove this function eventually because in the future I will need a spritesheet
/// for all 8 directions instead of relying on flipping the sprite.
/// Because the sprite needs to be flipped for NW, W, and SW, this function will
/// return the opposite direction for these directions so that the sprite can be flipped properly
pub fn direction_to_texture_atlas_direction(vector: Vec2) -> Direction {
    match vector.try_normalize() {
        Some(normalized_vector) => {
            let x = normalized_vector.x;
            let y = normalized_vector.y;

            if y >= 0.5 && x.abs() <= 0.5 {
                Direction::North
            } else if y <= -0.5 && x.abs() <= 0.5 {
                Direction::South
            } else if x >= 0.5 && y.abs() <= 0.5 {
                Direction::East
            } else if x <= -0.5 && y.abs() <= 0.5 {
                // If this is returned, it needs to be flipped
                Direction::East
            } else if x >= 0.5 && y >= 0.5 {
                Direction::NorthEast
            } else if x <= -0.5 && y >= 0.5 {
                // If this is returned, it needs to be flipped
                Direction::NorthEast
            } else if x >= 0.5 && y <= -0.5 {
                Direction::SouthEast
            } else if x <= -0.5 && y <= -0.5 {
                // If this is returned, it needs to be flipped
                Direction::SouthEast
            } else {
                Direction::East
            }
        }
        None => Direction::East,
    }
}
