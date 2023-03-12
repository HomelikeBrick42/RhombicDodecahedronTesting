use bevy::prelude::*;

mod utils;

use utils::*;

pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::Windowed,
                resolution: bevy::window::WindowResolution::new(640.0, 480.0),
                title: "Rhombic Dodecahedron Grid".into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(camera_controls.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new(std::time::Duration::from_millis(10)));
    }
}

#[derive(Component)]
struct CameraProperties {
    movement_speed: f32,
    rotation_speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(rhombic_dodecahedron()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraProperties {
            movement_speed: 2.0,
            rotation_speed: 90.0f32.to_radians(),
        },
    ));
}

fn camera_controls(
    mut query: Query<(&mut Transform, &CameraProperties)>,
    time_step: Res<FixedTime>,
    input: Res<Input<KeyCode>>,
) {
    let ts = time_step.period.as_secs_f32();

    for (mut transform, camera) in &mut query {
        let transform = transform.as_mut();

        if input.pressed(KeyCode::W) {
            transform.translation += transform.forward() * (camera.movement_speed * ts);
        }
        if input.pressed(KeyCode::S) {
            transform.translation -= transform.forward() * (camera.movement_speed * ts);
        }

        if input.pressed(KeyCode::A) {
            transform.translation -= transform.right() * (camera.movement_speed * ts);
        }
        if input.pressed(KeyCode::D) {
            transform.translation += transform.right() * (camera.movement_speed * ts);
        }

        if input.pressed(KeyCode::Space) {
            transform.translation += transform.up() * (camera.movement_speed * ts);
        }
        if input.pressed(KeyCode::LControl) {
            transform.translation -= transform.up() * (camera.movement_speed * ts);
        }

        if input.pressed(KeyCode::Up) {
            transform.rotate_local_x(camera.rotation_speed * ts);
        }
        if input.pressed(KeyCode::Down) {
            transform.rotate_local_x(-camera.rotation_speed * ts);
        }

        if input.pressed(KeyCode::Left) {
            transform.rotate_local_y(camera.rotation_speed * ts);
        }
        if input.pressed(KeyCode::Right) {
            transform.rotate_local_y(-camera.rotation_speed * ts);
        }

        if input.pressed(KeyCode::Q) {
            transform.rotate_local_z(camera.rotation_speed * ts);
        }
        if input.pressed(KeyCode::E) {
            transform.rotate_local_z(-camera.rotation_speed * ts);
        }
    }
}
