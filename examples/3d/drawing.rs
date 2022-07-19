//! This example shows how to draw figures/shapes.

use bevy::{
    drawing::{bevy_geometry::figure, Drawable},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        // transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// draw in immediate mode
fn update(mut commands: Commands) {
    // draw regular polygon
    let mut regular_polygon = figure::RegularPolygon {
        shape: shape::RegularPolygon {
            // sides: *counter / 10,
            ..default()
        },
        ..default()
    };
    regular_polygon.transform.scale *= Vec3::splat(3.0);
    regular_polygon.draw(&mut commands);

    // draw quad
    let mut quad = figure::Quad::default();
    quad.transform.scale *= Vec3::splat(1.5);
    quad.transform.translation += Vec3::X * 2.5;
    quad.draw(&mut commands);

    // draw disk
    let mut disk = figure::Disk::default();
    disk.transform.translation += Vec3::X * -2.5;
    disk.draw(&mut commands);
}
