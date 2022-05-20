use crate::mesh::{Indices, Mesh};
use bevy_math::*;
use wgpu::PrimitiveTopology;

/// A rectangle on the XY plane centered at the origin.
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    /// Full width and height of the rectangle.
    pub size: Vec2,
    /// Horizontally-flip the texture coordinates of the resulting mesh.
    pub flip: bool,
}

impl Default for Quad {
    fn default() -> Self {
        Quad::new(Vec2::ONE)
    }
}

impl Quad {
    pub fn new(size: Vec2) -> Self {
        Self { size, flip: false }
    }

    pub fn flipped(size: Vec2) -> Self {
        Self { size, flip: true }
    }
}

impl From<Quad> for Mesh {
    fn from(quad: Quad) -> Self {
        let extent_x = quad.size.x / 2.0;
        let extent_y = quad.size.y / 2.0;

        let (u_left, u_right) = if quad.flip { (1.0, 0.0) } else { (0.0, 1.0) };
        let vertices = [
            ([-extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
            ([-extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [u_left, 0.0]),
            ([extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [u_right, 0.0]),
            ([extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [u_right, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        for (position, normal, uv) in &vertices {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
