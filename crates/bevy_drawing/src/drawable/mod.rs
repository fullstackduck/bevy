mod disk;
mod line;
mod quad;
mod regular_polygon;

use bevy_app::{App, CoreStage, PluginGroup, PluginGroupBuilder};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::Component,
    system::{Commands, ResMut},
};
use bevy_pbr::{Material, MaterialMeshBundle};

use bevy_render::mesh::Mesh;
pub(crate) use disk::DiskPlugin;
pub(crate) use quad::QuadPlugin;

use crate::system;

use self::{regular_polygon::RegularPolygonPlugin, line::LinePlugin};

fn add_drawable<D: Drawable>(app: &mut App) {
    app.add_system_to_stage(CoreStage::Last, system::spawn::<D>);
    // TODO: PostUpdate better?
}

pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(DiskPlugin);
        group.add(LinePlugin);
        group.add(RegularPolygonPlugin);
        group.add(QuadPlugin);
    }
}

/// This is like public representation for something that can be drawn
/// Private representation can be "Draw" trait (or structure?)
pub trait Drawable: Send + Sync + 'static {
    type DrawingMaterial: Material;

    // TODO: Implement inline
    fn draw(&self, commands: &mut Commands) -> &Self;

    fn bundle(
        &self,
        materials: &mut ResMut<Assets<Self::DrawingMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> MaterialMeshBundle<Self::DrawingMaterial>;

    // fn bundle(
    //     &self,
    //     commands: Commands,
    //     entity: Entity,
    //     materials: &mut ResMut<Assets<impl DrawingMaterial>>,
    //     meshes: &mut ResMut<Assets<Mesh>>,
    // );
}
// Is "DrawCommand" better name, probably not since it's a component?
#[derive(Component, Clone, Copy)]
pub struct Draw<T: Drawable> {
    pub figure: T,
    pub is_drawn: bool,
}

impl<T: Drawable> Draw<T> {
    fn from_figure(figure: T) -> Self {
        Draw {
            figure,
            is_drawn: false,
        }
    }
}

// Diagram is a collection of drawables like a parent child entity relation, so use parent/child entities instead of Vec impl?
// #[derive(Component, Clone, Copy)]
// pub struct Diagram {
//     drawables: Vec<Entity>,
// }

#[derive(Component, Clone, Copy)]
pub struct DrawStatus(pub bool);

impl Default for DrawStatus {
    fn default() -> Self {
        Self(Default::default())
    }
}
