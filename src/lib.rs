use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};

const TIME_STEP: f32 = 1.0 / 60.0;

const CAMERA_SPEED: f32 = 2.0;
const CAMERA_ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_2;

pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(640.0, 480.0),
                title: "Rhombic Dodecahedron Grid".into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(camera_move.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP));
    }
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
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_move(mut query: Query<&mut Transform, With<Camera3d>>, input: Res<Input<KeyCode>>) {
    for mut transform in &mut query {
        let transform = transform.as_mut();

        if input.pressed(KeyCode::W) {
            transform.translation += transform.forward() * (CAMERA_SPEED * TIME_STEP);
        }
        if input.pressed(KeyCode::S) {
            transform.translation -= transform.forward() * (CAMERA_SPEED * TIME_STEP);
        }

        if input.pressed(KeyCode::A) {
            transform.translation -= transform.right() * (CAMERA_SPEED * TIME_STEP)
        }
        if input.pressed(KeyCode::D) {
            transform.translation += transform.right() * (CAMERA_SPEED * TIME_STEP);
        }

        if input.pressed(KeyCode::Space) {
            transform.translation += transform.up() * (CAMERA_SPEED * TIME_STEP);
        }
        if input.pressed(KeyCode::LControl) {
            transform.translation -= transform.up() * (CAMERA_SPEED * TIME_STEP);
        }

        if input.pressed(KeyCode::Up) {
            transform.rotate_local_x(CAMERA_ROTATION_SPEED * TIME_STEP);
        }
        if input.pressed(KeyCode::Down) {
            transform.rotate_local_x(-CAMERA_ROTATION_SPEED * TIME_STEP);
        }

        if input.pressed(KeyCode::Left) {
            transform.rotate_local_y(CAMERA_ROTATION_SPEED * TIME_STEP);
        }
        if input.pressed(KeyCode::Right) {
            transform.rotate_local_y(-CAMERA_ROTATION_SPEED * TIME_STEP);
        }

        if input.pressed(KeyCode::Q) {
            transform.rotate_local_z(CAMERA_ROTATION_SPEED * TIME_STEP);
        }
        if input.pressed(KeyCode::E) {
            transform.rotate_local_z(-CAMERA_ROTATION_SPEED * TIME_STEP);
        }
    }
}
