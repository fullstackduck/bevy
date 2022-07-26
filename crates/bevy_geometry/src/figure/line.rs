use bevy_math::Vec3;
use bevy_render::prelude::Color;
use bevy_transform::prelude::Transform;
use bevy_utils::default;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub width: f32,
    pub color: Color,
    // TODO: additional properties like line color, color mode, thickness
}

impl Default for Line {
    fn default() -> Self {
        let material = Self {
            start: Vec3::ZERO,
            end: Vec3::X,
            width: 0.003,
            color: Default::default(),
        };

        // let dir = material.end - material.start;
        // let normal = Vec3::new(-dir.y, dir.x, 0.0).normalize_or_zero();
        // println!(
        //     "{}, {} : {} , {}",
        //     material.start - normal,
        //     material.start + normal,
        //     material.end - normal,
        //     material.end + normal
        // );

        material
    }
}

// TODO: Shape for line figure?
