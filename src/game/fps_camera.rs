use bevy::{input::mouse::MouseMotion, prelude::*};

use super::lock_cursor::CursorLockState;

#[derive(Component)]
pub struct FPSCamera {
    pub sensitivity: f32,
    pub rotation: Vec3,
}

impl Default for FPSCamera {
    fn default() -> Self {
        FPSCamera {
            sensitivity: 0.001,
            rotation: Vec3::ZERO,
        }
    }
}

pub fn move_camera(
    cursor_lock_state: Res<CursorLockState>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut FPSCamera, &mut Transform)>,
) {
    if !cursor_lock_state.state { return; }

    for (mut camera, mut transform) in camera_query.iter_mut() {
        for ev in motion_evr.read() {
            camera.rotation.x -= ev.delta.y * camera.sensitivity;
            camera.rotation.y -= ev.delta.x * camera.sensitivity;
            camera.rotation.x = camera.rotation.x.clamp(-1.5, 1.5);
        }
        let x_quat = Quat::from_rotation_x(camera.rotation.x);
        let y_quat = Quat::from_rotation_y(camera.rotation.y);

        transform.rotation = y_quat * x_quat;
    }
}
