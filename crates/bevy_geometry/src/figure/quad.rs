use bevy_render::{mesh::shape, prelude::Color};
use bevy_transform::prelude::Transform;
use bevy_utils::default;

#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub shape: shape::Quad,
    pub transform: Transform,
    pub color: Color,
    // TODO: additional properties like line color, color mode, thickness
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            transform: Default::default(),
            color: Default::default(),
        }
    }
}

impl From<shape::Quad> for Quad {
    fn from(shape: shape::Quad) -> Self {
        Quad { shape, ..default() }
    }
}
