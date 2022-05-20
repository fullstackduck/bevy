use crate::mesh::{Indices, Mesh};
use wgpu::PrimitiveTopology;

/// A square on the XZ plane centered at the origin.
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    /// The total side length of the square.
    pub size: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Plane { size: 1.0 }
    }
}

impl From<Plane> for Mesh {
    fn from(plane: Plane) -> Self {
        let extent = plane.size / 2.0;

        let vertices = [
            ([extent, 0.0, -extent], [0.0, 1.0, 0.0], [1.0, 1.0]),
            ([extent, 0.0, extent], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([-extent, 0.0, extent], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([-extent, 0.0, -extent], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
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
