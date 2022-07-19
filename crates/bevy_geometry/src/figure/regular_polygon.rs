use bevy_render::{mesh::shape, prelude::Color};
use bevy_transform::prelude::Transform;
use bevy_utils::default;

#[derive(Debug, Copy, Clone)]
pub struct RegularPolygon {
    pub shape: shape::RegularPolygon,
    pub transform: Transform,
    pub color: Color,
    // TODO: additional properties like line color, color mode, thickness
}

impl Default for RegularPolygon {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            transform: Default::default(),
            color: Default::default(),
        }
    }
}

impl From<shape::RegularPolygon> for RegularPolygon {
    fn from(shape: shape::RegularPolygon) -> Self {
        RegularPolygon { shape, ..default() }
    }
}
