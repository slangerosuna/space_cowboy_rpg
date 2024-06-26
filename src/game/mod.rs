use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod fps_camera;
mod lock_cursor;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(lock_cursor::CursorLockState {
                state: false,
                allow_lock: true,
            })
            .add_systems(Startup, setup_scene)
            .add_systems(Update, lock_cursor::lock_cursor_position)
            .add_systems(Update, fps_camera::move_camera);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(fps_camera::FPSCamera::default());

    commands.spawn( PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 10000.0,
            range: 20.0,
            ..Default::default()
        },
        ..default()
    });

    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y)),
            material: materials.add(
                StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 0.5),
                        ..Default::default()
                }
            ),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..Default::default()
        });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.9))
        .insert(PbrBundle {
            mesh: meshes.add(Circle::new(0.5)),
            material: materials.add(
                StandardMaterial {
                    base_color: Color::rgb(0.8, 0.7, 0.6),
                    perceptual_roughness: 0.7,
                    reflectance: 0.5,
                    ..Default::default()
                }
            ),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..Default::default()
        });
}