use bevy::{ecs::system::SystemState, prelude::*};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin, EguiSet},
    egui,
};

mod displayable_component;
mod utils;

use displayable_component::*;
use utils::*;

pub struct GamePlugin;

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

        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn DisplayableComponent, Transform>()
            .register_component_as::<dyn DisplayableComponent, GlobalTransform>()
            .register_component_as::<dyn DisplayableComponent, Handle<Mesh>>()
            .register_component_as::<dyn DisplayableComponent, Handle<StandardMaterial>>()
            .register_component_as::<dyn DisplayableComponent, Visibility>()
            .register_component_as::<dyn DisplayableComponent, ComputedVisibility>();
    }
}

#[derive(Resource)]
struct UISettings {
    settings_window_open: bool,
    entities_window_open: bool,
}

#[derive(Component)]
pub struct ShowInUIProperties {
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
            name: self.name.clone() + " Copy",
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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        ShowInUIProperties::new("Ground".to_string()),
    ));

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

fn draw_ui(world: &mut World) {
    let ctx = SystemState::<EguiContexts>::from_world(world)
        .get_mut(world)
        .ctx_mut()
        .clone();

    egui::TopBottomPanel::top("Top Panel").show(&ctx, |ui| {
        let mut settings = world.get_resource_mut::<UISettings>().unwrap();
        ui.horizontal(|ui| {
            if ui.button("Settings").clicked() {
                settings.settings_window_open = true;
            }
            if ui.button("Entities").clicked() {
                settings.entities_window_open = true;
            }
        });
    });

    let mut settings_window_open = world
        .get_resource_mut::<UISettings>()
        .unwrap()
        .settings_window_open;
    egui::Window::new("Settings")
        .open(&mut settings_window_open)
        .vscroll(true)
        .show(&ctx, |ui| {
            let mut time_step = world.get_resource_mut::<FixedTime>().unwrap();
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
    world
        .get_resource_mut::<UISettings>()
        .unwrap()
        .settings_window_open = settings_window_open;

    let mut entities_window_open = world
        .get_resource_mut::<UISettings>()
        .unwrap()
        .entities_window_open;
    egui::Window::new("Entities")
        .open(&mut entities_window_open)
        .vscroll(true)
        .show(&ctx, |ui| {
            let entities = world
                .query_filtered::<Entity, With<ShowInUIProperties>>()
                .iter_mut(world)
                .collect::<Vec<_>>();
            for entity in entities {
                let ui_properties = world.get::<ShowInUIProperties>(entity).unwrap();
                egui::CollapsingHeader::new(&ui_properties.name)
                    .id_source(entity)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Name: ");
                            let mut ui_properties =
                                world.get_mut::<ShowInUIProperties>(entity).unwrap();
                            ui.text_edit_singleline(&mut ui_properties.name);
                        });

                        {
                            let mut system_state = SystemState::<(
                                Commands,
                                Query<(&mut ShowInUIProperties, &mut dyn DisplayableComponent)>,
                            )>::from_world(world);
                            let (mut commands, mut query) = system_state.get_mut(world);

                            let (mut ui_properties, mut displayable_components) =
                                query.get_mut(entity).unwrap();

                            for mut displayable_component in &mut displayable_components {
                                let name = displayable_component.get_name();
                                let mut remove = false;
                                ui.collapsing(name, |ui| {
                                    displayable_component.show_ui(
                                        entity,
                                        ui_properties.as_mut(),
                                        ui,
                                        &mut World::new(), // somehow pass the actual world in here
                                    );
                                    remove |= ui.button("Remove").clicked();
                                });
                                if remove {
                                    let mut entity_commands = commands.entity(entity);
                                    displayable_component.remove_component(&mut entity_commands);
                                }
                            }
                        }

                        if ui.button("Duplicate").clicked() {
                            let mut system_state = SystemState::<(
                                Commands,
                                Query<(&ShowInUIProperties, &mut dyn DisplayableComponent)>,
                            )>::from_world(world);
                            let (mut commands, mut query) = system_state.get_mut(world);

                            let (ui_properties, mut displayable_components) =
                                query.get_mut(entity).unwrap();

                            let ui_properties = ui_properties.clone();

                            let mut entity_commands = commands.spawn(ui_properties);
                            for displayable_component in &mut displayable_components {
                                displayable_component.clone_onto(&mut entity_commands);
                            }
                        }
                    });
            }
            ui.allocate_space(ui.available_size());
        });
    world
        .get_resource_mut::<UISettings>()
        .unwrap()
        .entities_window_open = entities_window_open;
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
