use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::fps_camera::FPSCamera;

#[derive(Component)]
pub struct FPSMovement {
    pub speed: f32,
    pub power: f32,
}

pub fn player_movement(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut movement_query: Query<(
        &Transform,
        &mut Damping,
        &FPSCamera,
        &FPSMovement,
        &mut Velocity,
    )>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let res = movement_query.iter_mut().next();
    if res.is_none() {
        return;
    }
    let (transform, mut damping, fps_camera, fps_movement, mut velocity) = res.unwrap();

    let mut dir = Vec2::new(0.0, 0.0);

    let mut air_mod = 1.0;

    let hit = rapier_context.cast_ray(
        transform.translation - 0.5,
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        true,
        QueryFilter::only_fixed(),
    );
    let grounded = hit.is_some();

    if grounded {
        damping.linear_damping = 15.0;
    } else {
        damping.linear_damping = 0.0;
        air_mod = 0.05;
    }

    if key.pressed(KeyCode::KeyW) {
        dir.y -= 1.0;
    }
    if key.pressed(KeyCode::KeyS) {
        dir.y += 1.0;
    }
    if key.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if key.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    dir = Vec2::new(
        dir.x * f32::cos(-fps_camera.rotation.y) - dir.y * f32::sin(-fps_camera.rotation.y),
        dir.x * f32::sin(-fps_camera.rotation.y) + dir.y * f32::cos(-fps_camera.rotation.y),
    );

    let mut halt = false;
    if dir.length() > 0.0 {
        dir = dir.normalize();
    } else if grounded {
        let len_sq = velocity.linvel.xz().length_squared();
        if len_sq > 4.0 {
            dir = -velocity.linvel.xz().normalize();
        } else {
            halt = true;
        }
    }

    if key.just_pressed(KeyCode::Space) {
        if grounded {
            velocity.linvel.y = 4.;
        }
    }
    if halt {
        velocity.linvel.x = 0.0;
        velocity.linvel.z = 0.0;
        return;
    }
    let vel = velocity.linvel.length();
    let mult = 1.0 / if vel > 0.1 { vel } else { 0.1 };
    velocity.linvel.x += dir.x * fps_movement.power * mult * time.delta_seconds() * air_mod;
    velocity.linvel.z += dir.y * fps_movement.power * mult * time.delta_seconds() * air_mod;

    let net_velocity = Vec2::new(velocity.linvel.x, velocity.linvel.z).length();
    let multiplier;
    if net_velocity > fps_movement.speed {
        multiplier = fps_movement.speed / net_velocity;
    } else {
        multiplier = 1.0;
    }

    velocity.linvel.x *= multiplier;
    velocity.linvel.z *= multiplier;
}
