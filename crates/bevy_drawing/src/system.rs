use bevy_asset::{Assets, Handle};
use bevy_ecs::{
    entity::Entity,
    prelude::Without,
    system::{Commands, Local, Query, ResMut},
};
use bevy_render::mesh::Mesh;

use crate::{Draw, DrawStatus, Drawable};

// TODO: despawn_batch()
pub(crate) fn despawn(mut commands: Commands, mut query: Query<(Entity, &mut DrawStatus)>) {
    for (entity, mut draw_status) in query.iter_mut() {
        if !draw_status.0 {
            draw_status.0 = true;
            return;
        }

        commands.entity(entity).despawn();
    }
}

// TODO: Some way to have single spawn system run for all Drawables, Assets<M::DrawingMaterial> is blocking
pub(crate) fn spawn<M: Drawable>(
    mut commands: Commands,
    mut materials: ResMut<Assets<M::DrawingMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut previous_len: Local<usize>,
    mut query: Query<(Entity, &Draw<M>), Without<Handle<Mesh>>>,
) {
    let mut values = Vec::with_capacity(*previous_len);

    for (entity, drawable) in query.iter_mut() {
        values.push((entity, drawable.figure.bundle(&mut materials, &mut meshes)));
        commands.entity(entity).insert(DrawStatus::default());
    }

    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}
