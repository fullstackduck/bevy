use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Assets, HandleUntyped};
use bevy_ecs::system::{Commands, ResMut};
use bevy_geometry::figure;
use bevy_pbr::{Material, MaterialMeshBundle, MaterialPlugin};
use bevy_reflect::TypeUuid;
use bevy_render::{
    mesh::{shape, Mesh},
    prelude::{Color, Shader},
    render_resource::{AsBindGroup, ShaderRef},
};
use bevy_utils::default;

use crate::{drawable::add_drawable, Draw, Drawable};

/// Handle to the custom shader with a unique random ID
pub const DISK_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845418412094821);

#[derive(Default)]
pub struct DiskPlugin;

impl Plugin for DiskPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            DISK_SHADER_HANDLE,
            "../shaders/disk.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(MaterialPlugin::<DiskMaterial>::default());

        add_drawable::<figure::Disk>(app);
    }
}

impl Drawable for figure::Disk {
    type DrawingMaterial = DiskMaterial;

    fn draw(&self, commands: &mut Commands) -> &Self {
        commands.spawn(Draw::from_figure(self.clone()));
        self
    }

    fn bundle(
        &self,
        materials: &mut ResMut<Assets<Self::DrawingMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> MaterialMeshBundle<Self::DrawingMaterial> {
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())),
            transform: self.transform,
            material: materials.add(DiskMaterial { color: self.color }),
            ..default()
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct DiskMaterial {
    #[uniform(1)]
    color: Color,
}

impl Material for DiskMaterial {
    fn fragment_shader() -> ShaderRef {
        DISK_SHADER_HANDLE.typed::<Shader>().into()
    }
}
