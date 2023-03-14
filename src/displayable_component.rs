use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::egui;

use crate::ShowInUIProperties;

#[bevy_trait_query::queryable]
pub trait DisplayableComponent {
    fn get_name(&self) -> &'static str;
    fn clone_onto(&self, commands: &mut EntityCommands);
    fn remove_component(&mut self, commands: &mut EntityCommands);
    fn show_ui(
        &mut self,
        entity: Entity,
        ui_properties: &mut ShowInUIProperties,
        ui: &mut egui::Ui,
        world: &mut World,
    );
}

impl DisplayableComponent for Transform {
    fn get_name(&self) -> &'static str {
        "Transform"
    }

    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
    }

    fn remove_component(&mut self, commands: &mut EntityCommands) {
        commands.remove::<Self>();
    }

    fn show_ui(
        &mut self,
        _entity: Entity,
        ui_properties: &mut ShowInUIProperties,
        ui: &mut egui::Ui,
        _world: &mut World,
    ) {
        render_transform_fields(self, ui_properties, ui, true);
    }
}

impl DisplayableComponent for GlobalTransform {
    fn get_name(&self) -> &'static str {
        "Global Transform"
    }

    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
    }

    fn remove_component(&mut self, commands: &mut EntityCommands) {
        commands.remove::<Self>();
    }

    fn show_ui(
        &mut self,
        _entity: Entity,
        ui_properties: &mut ShowInUIProperties,
        ui: &mut egui::Ui,
        _world: &mut World,
    ) {
        render_transform_fields(&mut self.compute_transform(), ui_properties, ui, false);
    }
}

fn render_transform_fields(
    transform: &mut Transform,
    ui_properties: &mut ShowInUIProperties,
    ui: &mut egui::Ui,
    enabled: bool,
) {
    ui.add_enabled_ui(enabled, |ui| {
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
            let mut euler_angles = {
                let (y, x, z) = transform.rotation.to_euler(EulerRot::YXZ);
                Vec3 {
                    x: x.to_degrees(),
                    y: y.to_degrees(),
                    z: z.to_degrees(),
                }
            };
            let euler_angles = if enabled {
                ui_properties.euler_angles_cache.get_or_insert(euler_angles)
            } else {
                &mut euler_angles
            };

            let x_response = ui.add(egui::DragValue::new(&mut euler_angles.x).prefix("x: "));
            let y_response = ui.add(egui::DragValue::new(&mut euler_angles.y).prefix("y: "));
            let z_response = ui.add(egui::DragValue::new(&mut euler_angles.z).prefix("z: "));
            if x_response.changed() || y_response.changed() || z_response.changed() {
                transform.rotation = Quat::from_euler(
                    EulerRot::YXZ,
                    euler_angles.y.to_radians(),
                    euler_angles.x.to_radians(),
                    euler_angles.z.to_radians(),
                );
            }
            if enabled
                && ((!x_response.has_focus() && !x_response.dragged())
                    && (!y_response.has_focus() && !y_response.dragged())
                    && (!z_response.has_focus() && !z_response.dragged()))
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
        ui.add_enabled_ui(false, |ui| {
            ui.horizontal(|ui| {
                ui.label("Forward: ");
                let mut forward = transform.forward();
                ui.add(egui::DragValue::new(&mut forward.x).prefix("x: "));
                ui.add(egui::DragValue::new(&mut forward.y).prefix("y: "));
                ui.add(egui::DragValue::new(&mut forward.z).prefix("z: "));
            });
            ui.horizontal(|ui| {
                ui.label("Right: ");
                let mut right = transform.right();
                ui.add(egui::DragValue::new(&mut right.x).prefix("x: "));
                ui.add(egui::DragValue::new(&mut right.y).prefix("y: "));
                ui.add(egui::DragValue::new(&mut right.z).prefix("z: "));
            });
            ui.horizontal(|ui| {
                ui.label("Up: ");
                let mut up = transform.up();
                ui.add(egui::DragValue::new(&mut up.x).prefix("x: "));
                ui.add(egui::DragValue::new(&mut up.y).prefix("y: "));
                ui.add(egui::DragValue::new(&mut up.z).prefix("z: "));
            });
        });
    });
}

impl DisplayableComponent for Handle<Mesh> {
    fn get_name(&self) -> &'static str {
        "Mesh"
    }

    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(self.clone());
    }

    fn remove_component(&mut self, commands: &mut EntityCommands) {
        commands.remove::<Self>();
    }

    fn show_ui(
        &mut self,
        _entity: Entity,
        _ui_properties: &mut ShowInUIProperties,
        _ui: &mut egui::Ui,
        _world: &mut World,
    ) {
    }
}

impl DisplayableComponent for Handle<StandardMaterial> {
    fn get_name(&self) -> &'static str {
        "StandardMaterial"
    }

    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(self.clone());
    }

    fn remove_component(&mut self, commands: &mut EntityCommands) {
        commands.remove::<Self>();
    }

    fn show_ui(
        &mut self,
        _entity: Entity,
        _ui_properties: &mut ShowInUIProperties,
        _ui: &mut egui::Ui,
        _world: &mut World,
    ) {
    }
}

impl DisplayableComponent for Visibility {
    fn get_name(&self) -> &'static str {
        "Visibility"
    }

    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
    }

    fn remove_component(&mut self, commands: &mut EntityCommands) {
        commands.remove::<Self>();
    }

    fn show_ui(
        &mut self,
        entity: Entity,
        _ui_properties: &mut ShowInUIProperties,
        ui: &mut egui::Ui,
        _world: &mut World,
    ) {
        egui::ComboBox::new(entity, "")
            .selected_text(match self {
                Visibility::Inherited => "Inherited",
                Visibility::Hidden => "Hidden",
                Visibility::Visible => "Visible",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(self, Visibility::Inherited, "Inherited");
                ui.selectable_value(self, Visibility::Hidden, "Hidden");
                ui.selectable_value(self, Visibility::Visible, "Visible");
            });
    }
}

impl DisplayableComponent for ComputedVisibility {
    fn get_name(&self) -> &'static str {
        "ComputedVisibility"
    }

    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(self.clone());
    }

    fn remove_component(&mut self, commands: &mut EntityCommands) {
        commands.remove::<Self>();
    }

    fn show_ui(
        &mut self,
        _entity: Entity,
        _ui_properties: &mut ShowInUIProperties,
        ui: &mut egui::Ui,
        _world: &mut World,
    ) {
        if self.is_visible() {
            ui.label("Visible");
        } else {
            ui.label("Not Visible");
        }
    }
}
