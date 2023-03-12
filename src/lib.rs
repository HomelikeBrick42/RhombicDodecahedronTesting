use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology,
    window::{WindowMode, WindowResolution},
};

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
        .add_system(camera_controls.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP));
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;

const CAMERA_SPEED: f32 = 2.0;
const CAMERA_ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_2;

#[derive(Component)]
struct MainCamera;

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

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    {
        let mut vertices = vec![];
        let single_face = [
            // Front Top
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.0, 0.0, 1.0],
            // Front Bottom
            [0.5, -0.5, 0.5],
            [0.0, 0.0, 1.0],
            [-0.5, -0.5, 0.5],
            // Front Left
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.0, 0.0, 1.0],
            // Front Right
            [0.5, 0.5, 0.5],
            [0.0, 0.0, 1.0],
            [0.5, -0.5, 0.5],
        ];
        for rotation in [
            Quat::from_rotation_y(0.0f32.to_radians()),
            Quat::from_rotation_y(90.0f32.to_radians()),
            Quat::from_rotation_y(-90.0f32.to_radians()),
            Quat::from_rotation_y(-180.0f32.to_radians()),
            Quat::from_rotation_x(90.0f32.to_radians()),
            Quat::from_rotation_x(-90.0f32.to_radians()),
        ] {
            vertices.extend(
                single_face
                    .into_iter()
                    .map(|[x, y, z]| Transform::from_rotation(rotation) * Vec3 { x, y, z }),
            );
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.compute_flat_normals();
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
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
        MainCamera,
    ));
}

fn camera_controls(mut query: Query<&mut Transform, With<MainCamera>>, input: Res<Input<KeyCode>>) {
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
