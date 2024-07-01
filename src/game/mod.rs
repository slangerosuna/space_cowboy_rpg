use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod fps_camera;
mod fps_movement;
mod lock_cursor;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(lock_cursor::CursorLockState {
            state: false,
            allow_lock: true,
        })
        .add_systems(Startup, setup_scene)
        .add_systems(Update, lock_cursor::lock_cursor_position)
        .add_systems(Update, fps_camera::move_camera)
        .add_systems(Update, fps_movement::player_movement);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 10000.0,
            range: 20.0,
            ..Default::default()
        },
        ..default()
    });

    commands
        .spawn(Collider::cuboid(1.0, 0.01, 1.0))
        .insert(PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::splat(10.0)),
            ..Default::default()
        });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.9))
        .insert(PbrBundle {
            mesh: meshes.add(Circle::new(0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                perceptual_roughness: 0.7,
                reflectance: 0.5,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..Default::default()
        });

    commands
        .spawn(SpatialBundle {
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .with_children(|child| {
            child.spawn((
                Visibility::Visible,
                Camera3dBundle {
                    camera: Camera {
                        hdr: false, //true, // 1. HDR is required for bloom
                        ..default()
                    },
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: (103.0 / 360.0) * (std::f32::consts::PI * 2.0),
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(0.0, 1.0, 4.0),

                    ..default()
                },
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED_X
                    | LockedAxes::ROTATION_LOCKED_Y
                    | LockedAxes::ROTATION_LOCKED_Z,
                Velocity {
                    linvel: Vec3::new(0.0, 0.0, 0.0),
                    angvel: Vec3::new(0.0, 0.0, 0.0),
                },
                Collider::cuboid(0.2, 1.4, 0.2),
                fps_camera::FPSCamera {
                    rotation: Vec3::new(0., 0., 0.),
                    sensitivity: (0.173) / 300.0,
                },
                Damping {
                    linear_damping: 4.,
                    angular_damping: 1.0,
                },
                GravityScale(1.),
                fps_movement::FPSMovement {
                    speed: 5.6,
                    power: 300.0,
                },
            ));
        });
}
