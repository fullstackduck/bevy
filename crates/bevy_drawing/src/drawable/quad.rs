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
pub const QUAD_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845428412094821);

#[derive(Default)]
pub struct QuadPlugin;

impl Plugin for QuadPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            QUAD_SHADER_HANDLE,
            "../shaders/quad.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(MaterialPlugin::<QuadMaterial>::default());

        add_drawable::<figure::Quad>(app);
    }
}

impl Drawable for figure::Quad {
    type DrawingMaterial = QuadMaterial;

    fn draw(&self, commands: &mut Commands) -> &Self {
        commands.spawn().insert(Draw::from_figure(self.clone()));
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
            material: materials.add(QuadMaterial { color: self.color }),
            ..default()
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a1f056e0"]
pub struct QuadMaterial {
    #[uniform(1)]
    color: Color,
}

impl Material for QuadMaterial {
    fn fragment_shader() -> ShaderRef {
        QUAD_SHADER_HANDLE.typed::<Shader>().into()
    }
}
