//! This example shows how to draw figures/shapes.

use bevy::{
    drawing::{bevy_geometry::figure, Drawable},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        // transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
}

/// draw in immediate mode
fn update(mut commands: Commands) {
    // draw regular polygon
    // let mut regular_polygon = figure::RegularPolygon {
    //     shape: shape::RegularPolygon {
    //         // sides: *counter / 10,
    //         ..default()
    //     },
    //     ..default()
    // };
    // regular_polygon.transform.scale *= Vec3::splat(3.0);
    // regular_polygon.draw(&mut commands);

    // // draw quad
    // let mut quad = figure::Quad::default();
    // quad.transform.scale *= Vec3::splat(1.5);
    // quad.transform.translation += Vec3::X * 2.5;
    // quad.draw(&mut commands);

    // // draw disk
    // let mut disk = figure::Disk::default();
    // disk.transform.translation += Vec3::X * -2.5;
    // disk.draw(&mut commands);

    // draw line
    // _
    let mut horizontal_line = figure::Line::default();
    horizontal_line.start = horizontal_line.end * 0.01;
    horizontal_line.draw(&mut commands);

    // |
    let mut vertical_line = figure::Line {
        end: Vec3::Y,
        ..default()
    };
    vertical_line.start = vertical_line.end * 0.01;
    vertical_line.draw(&mut commands);

    // /
    let mut diagonal_line = figure::Line {
        end: Vec3::X + Vec3::Y,
        ..default()
    };
    diagonal_line.start = diagonal_line.end * 0.01;
    diagonal_line.draw(&mut commands);

    // \
    let mut diagonal_3d_line = figure::Line {
        end: Vec3::new(-1.0, 1.0, -1.0),
        ..default()
    };
    diagonal_3d_line.start = diagonal_3d_line.end * 0.01;
    diagonal_3d_line.draw(&mut commands);
}
