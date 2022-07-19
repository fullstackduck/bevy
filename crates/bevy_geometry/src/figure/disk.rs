use bevy_render::{prelude::Color, mesh::shape};
use bevy_transform::prelude::Transform;
use bevy_utils::default;

#[derive(Debug, Copy, Clone)]
pub struct Disk {
    pub shape: shape::Disk,
    pub transform: Transform,
    pub color: Color,
    // TODO: additional properties like line color, color mode, thickness
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            transform: Default::default(),
            color: Default::default(),
        }
    }
}

impl From<shape::Disk> for Disk {
    fn from(shape: shape::Disk) -> Self {
        Disk { shape, ..default() }
    }
}
