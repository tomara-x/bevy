//! This example demonstrates how to use the `Camera::viewport_to_world_2d` method.

use bevy::{color::palettes::basic::WHITE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
