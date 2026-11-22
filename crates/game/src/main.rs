use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::{CursorGrabMode, CursorOptions, PrimaryWindow}};
use engine::{RayCastCamera, RayCasterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RayCasterPlugin)
        .add_systems(Startup, (setup, capture_mouse))
        .add_systems(Update, move_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(RayCastCamera {
        position: Vec2::new(5.0, 5.0),
        direction: 0.0,
        fov: 2.0,
    });
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    mut camera: Single<&mut RayCastCamera>
) {
    let input_vector = Vec2::new(
        if keyboard_input.pressed(KeyCode::KeyW) {1.0} else {0.0} - if keyboard_input.pressed(KeyCode::KeyS) {1.0} else {0.0}, 
        if keyboard_input.pressed(KeyCode::KeyD) {1.0} else {0.0 - if keyboard_input.pressed(KeyCode::KeyA) {1.0} else {0.0}}
    );
    let input_vector = Vec2::from_angle(camera.direction).rotate(input_vector);
    let mouse = mouse_motion.delta;
    camera.position += input_vector * time.delta_secs();
    camera.direction += mouse.x * 0.01;
}

fn capture_mouse(mut windows: Query<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>) {
    // Query returns one window typically.
    for mut window in windows.iter_mut() {
        window.1.grab_mode = CursorGrabMode::Locked;
    }
}
