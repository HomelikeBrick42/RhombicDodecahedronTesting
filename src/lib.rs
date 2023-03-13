use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin, EguiSet},
    egui,
};

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
        //.add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup)
        .add_system(
            draw_ui
                .in_base_set(CoreSet::Update)
                .after(EguiSet::BeginFrame),
        )
        .add_system(camera_controls.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new(std::time::Duration::from_millis(10)))
        .insert_resource(AmbientLight {
            brightness: 0.05,
            ..default()
        })
        .insert_resource(UISettings {
            settings_window_open: false,
            entities_window_open: false,
        });
    }
}

#[derive(Resource)]
struct UISettings {
    settings_window_open: bool,
    entities_window_open: bool,
}

#[derive(Component)]
struct ShowInUIProperties {
    name: String,
    euler_angles_cache: Option<Vec3>,
}

impl ShowInUIProperties {
    pub fn new(name: String) -> Self {
        Self {
            name,
            euler_angles_cache: None,
        }
    }
}

impl Clone for ShowInUIProperties {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            euler_angles_cache: None,
        }
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
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        ShowInUIProperties::new("Rhombic Dodecahedron".to_string()),
    ));

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

fn draw_ui(
    mut contexts: EguiContexts,
    mut entities: Query<(Entity, &mut ShowInUIProperties, Option<&mut Transform>)>,
    mut settings: ResMut<UISettings>,
    mut time_step: ResMut<FixedTime>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Settings").clicked() {
                settings.settings_window_open = true;
            }
            if ui.button("Entities").clicked() {
                settings.entities_window_open = true;
            }
        });
    });

    egui::Window::new("Settings")
        .open(&mut settings.settings_window_open)
        .vscroll(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Time Step: ");
                let mut step = time_step.period.as_secs_f64();
                if ui
                    .add(
                        egui::DragValue::new(&mut step)
                            .suffix("s")
                            .speed(0.001)
                            .clamp_range(0.001..=1.0),
                    )
                    .changed()
                {
                    time_step.period = std::time::Duration::from_secs_f64(step);
                }
            });
            ui.allocate_space(ui.available_size());
        });

    egui::Window::new("Entities")
        .open(&mut settings.entities_window_open)
        .vscroll(true)
        .show(ctx, |ui| {
            for (entity, mut ui_properties, mut transform) in &mut entities {
                let mut duplicate = false;
                egui::CollapsingHeader::new(&ui_properties.name)
                    .id_source(entity)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Name: ");
                            ui.text_edit_singleline(&mut ui_properties.name);
                        });

                        if let Some(transform) = transform.as_mut() {
                            ui.horizontal(|ui| {
                                ui.label("Position: ");
                                ui.add(
                                    egui::DragValue::new(&mut transform.translation.x)
                                        .prefix("x: ")
                                        .speed(0.01),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut transform.translation.y)
                                        .prefix("y: ")
                                        .speed(0.01),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut transform.translation.z)
                                        .prefix("z: ")
                                        .speed(0.01),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("Rotation: ");
                                let euler_angles =
                                    ui_properties.euler_angles_cache.get_or_insert_with(|| {
                                        let (y, x, z) = transform.rotation.to_euler(EulerRot::YXZ);
                                        Vec3 {
                                            x: x.to_degrees(),
                                            y: y.to_degrees(),
                                            z: z.to_degrees(),
                                        }
                                    });

                                let x_response =
                                    ui.add(egui::DragValue::new(&mut euler_angles.x).prefix("x: "));
                                let y_response =
                                    ui.add(egui::DragValue::new(&mut euler_angles.y).prefix("y: "));
                                let z_response =
                                    ui.add(egui::DragValue::new(&mut euler_angles.z).prefix("z: "));
                                if x_response.changed()
                                    || y_response.changed()
                                    || z_response.changed()
                                {
                                    transform.rotation = Quat::from_euler(
                                        EulerRot::YXZ,
                                        euler_angles.y.to_radians(),
                                        euler_angles.x.to_radians(),
                                        euler_angles.z.to_radians(),
                                    );
                                }
                                if (!x_response.has_focus() && !x_response.dragged())
                                    && (!y_response.has_focus() && !y_response.dragged())
                                    && (!z_response.has_focus() && !z_response.dragged())
                                {
                                    ui_properties.euler_angles_cache = None;
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label("Scale: ");
                                ui.add(
                                    egui::DragValue::new(&mut transform.scale.x)
                                        .prefix("x: ")
                                        .speed(0.01),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut transform.scale.y)
                                        .prefix("y: ")
                                        .speed(0.01),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut transform.scale.z)
                                        .prefix("z: ")
                                        .speed(0.01),
                                );
                            });
                        }

                        duplicate |= ui.button("Duplicate").clicked();
                    });
                if duplicate {
                    // do nothing
                }
            }
            ui.allocate_space(ui.available_size());
        });
}

fn camera_controls(
    mut contexts: EguiContexts,
    mut query: Query<(&mut Transform, &mut CameraProperties), With<MainCamera>>,
    time_step: Res<FixedTime>,
    input: Res<Input<KeyCode>>,
) {
    let ts = time_step.period.as_secs_f32();

    if !contexts.ctx_mut().wants_keyboard_input() {
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
}
