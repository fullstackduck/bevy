use crate::mesh::Mesh;

use super::RegularPolygon;

/// A disk in the xy plane
#[derive(Debug, Copy, Clone)]
pub struct Disk {
    /// Inscribed radius in the xy plane.
    pub radius: f32,
    /// The number of vertices used.
    pub vertices: usize,
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            radius: 0.5,
            vertices: 64,
        }
    }
}

impl Disk {
    /// Creates a disk in the xy plane
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            ..Default::default()
        }
    }
}

impl From<Disk> for RegularPolygon {
    fn from(disk: Disk) -> Self {
        Self {
            radius: disk.radius,
            sides: disk.vertices,
        }
    }
}

impl From<Disk> for Mesh {
    fn from(disk: Disk) -> Self {
        Mesh::from(RegularPolygon::from(disk))
    }
}
