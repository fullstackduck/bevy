use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Assets, HandleUntyped};
use bevy_ecs::system::{Commands, ResMut};
use bevy_geometry::figure;
use bevy_math::prelude::*;
use bevy_pbr::{Material, MaterialMeshBundle, MaterialPlugin};
use bevy_reflect::TypeUuid;
use bevy_render::{
    mesh::{shape, Mesh},
    prelude::{Color, Shader},
    render_resource::{AsBindGroup, ShaderRef},
};
use bevy_transform::prelude::Transform;
use bevy_utils::default;

use crate::{drawable::add_drawable, Draw, Drawable};

/// Handle to the custom shader with a unique random ID
pub const LINE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845428412094811);

#[derive(Default)]
pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            LINE_SHADER_HANDLE,
            "../shaders/line.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(MaterialPlugin::<LineMaterial>::default());

        add_drawable::<figure::Line>(app);
    }
}

impl Drawable for figure::Line {
    type DrawingMaterial = LineMaterial;

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
            transform: Transform::from_translation(self.start),
            material: materials.add(LineMaterial {
                start: self.start,
                end: self.end,
                width: self.width,
                color: self.color,
            }),
            ..default()
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a1f156e0"]
pub struct LineMaterial {
    #[uniform(0)]
    start: Vec3,
    #[uniform(0)]
    end: Vec3,
    #[uniform(0)]
    width: f32,
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn vertex_shader() -> ShaderRef {
        LINE_SHADER_HANDLE.typed::<Shader>().into()
    }

    fn fragment_shader() -> ShaderRef {
        LINE_SHADER_HANDLE.typed::<Shader>().into()
    }
}
