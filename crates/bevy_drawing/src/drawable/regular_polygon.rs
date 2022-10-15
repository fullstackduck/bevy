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
pub const REGULARPOLYGON_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845418412194821);

#[derive(Default)]
pub struct RegularPolygonPlugin;

impl Plugin for RegularPolygonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            REGULARPOLYGON_SHADER_HANDLE,
            "../shaders/regular_polygon.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(MaterialPlugin::<RegularPolygonMaterial>::default());

        add_drawable::<figure::RegularPolygon>(app);
    }
}

impl Drawable for figure::RegularPolygon {
    type DrawingMaterial = RegularPolygonMaterial;

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
            material: materials.add(RegularPolygonMaterial { color: self.color }),
            ..default()
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f856e0"]
pub struct RegularPolygonMaterial {
    #[uniform(1)]
    color: Color,
}

impl Material for RegularPolygonMaterial {
    fn fragment_shader() -> ShaderRef {
        REGULARPOLYGON_SHADER_HANDLE.typed::<Shader>().into()
    }
}