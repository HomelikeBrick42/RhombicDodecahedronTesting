use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

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
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup)
        //.add_system(draw_ui.after(EguiSet::BeginFrame))
        .add_system(camera_controls.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new(std::time::Duration::from_millis(10)))
        .insert_resource(AmbientLight {
            brightness: 0.05,
            ..default()
        })
        .insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4096 });
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct CameraProperties {
    movement_speed: f32,
    rotation_speed: f32,
    pitch: f32,
    yaw: f32,
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

    let mesh = meshes.add(rhombic_dodecahedron());
    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 30000.0,
            ..default()
        },
        transform: Transform::default().looking_at(
            Vec3 {
                x: 0.3,
                y: -1.0,
                z: -0.4,
            },
            Vec3::Y,
        ),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.5, 5.0),
            ..default()
        },
        CameraProperties {
            movement_speed: 3.0,
            rotation_speed: 90.0f32.to_radians(),
            pitch: 0.0,
            yaw: 0.0,
        },
        MainCamera,
    ));
}

fn camera_controls(
    mut query: Query<(&mut Transform, &mut CameraProperties), With<MainCamera>>,
    time_step: Res<FixedTime>,
    input: Res<Input<KeyCode>>,
) {
    let ts = time_step.period.as_secs_f32();

    for (mut transform, mut camera) in &mut query {
        // Movement
        {
            let movement_speed = camera.movement_speed
                * if input.pressed(KeyCode::LShift) {
                    2.0
                } else {
                    1.0
                };

            let mut movement = Vec3::ZERO;
            if input.pressed(KeyCode::W) {
                movement += transform.forward();
            }
            if input.pressed(KeyCode::S) {
                movement -= transform.forward();
            }

            if input.pressed(KeyCode::A) {
                movement -= transform.right();
            }
            if input.pressed(KeyCode::D) {
                movement += transform.right();
            }

            if input.pressed(KeyCode::E) {
                movement += transform.up();
            }
            if input.pressed(KeyCode::Q) {
                movement -= transform.up();
            }
            transform.translation += movement.normalize_or_zero() * (movement_speed * ts);
        }

        // Rotation
        {
            let mut rotation_changed = false;
            if input.pressed(KeyCode::Up) {
                camera.pitch += camera.rotation_speed * ts;
                rotation_changed = true;
            }
            if input.pressed(KeyCode::Down) {
                camera.pitch -= camera.rotation_speed * ts;
                rotation_changed = true;
            }

            if input.pressed(KeyCode::Left) {
                camera.yaw += camera.rotation_speed * ts;
                rotation_changed = true;
            }
            if input.pressed(KeyCode::Right) {
                camera.yaw -= camera.rotation_speed * ts;
                rotation_changed = true;
            }

            if rotation_changed {
                camera.pitch = camera
                    .pitch
                    .clamp(-90.0f32.to_radians(), 90.0f32.to_radians());
                camera.yaw %= std::f32::consts::TAU;
                transform.rotation =
                    Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);
            }
        }
    }
}
