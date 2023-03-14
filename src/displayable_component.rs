use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::egui;

use crate::ShowInUIProperties;

#[bevy_trait_query::queryable]
pub trait DisplayableComponent {
    fn clone_onto(&self, commands: &mut EntityCommands);
    fn display(&mut self, ui_properties: &mut ShowInUIProperties, ui: &mut egui::Ui) {
        _ = ui_properties;
        _ = ui;
    }
}

impl DisplayableComponent for Transform {
    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
    }

    fn display(&mut self, ui_properties: &mut ShowInUIProperties, ui: &mut egui::Ui) {
        ui.collapsing("Transform", |ui| {
            ui.horizontal(|ui| {
                ui.label("Position: ");
                ui.add(
                    egui::DragValue::new(&mut self.translation.x)
                        .prefix("x: ")
                        .speed(0.01),
                );
                ui.add(
                    egui::DragValue::new(&mut self.translation.y)
                        .prefix("y: ")
                        .speed(0.01),
                );
                ui.add(
                    egui::DragValue::new(&mut self.translation.z)
                        .prefix("z: ")
                        .speed(0.01),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Rotation: ");
                let euler_angles = ui_properties.euler_angles_cache.get_or_insert_with(|| {
                    let (y, x, z) = self.rotation.to_euler(EulerRot::YXZ);
                    Vec3 {
                        x: x.to_degrees(),
                        y: y.to_degrees(),
                        z: z.to_degrees(),
                    }
                });

                let x_response = ui.add(egui::DragValue::new(&mut euler_angles.x).prefix("x: "));
                let y_response = ui.add(egui::DragValue::new(&mut euler_angles.y).prefix("y: "));
                let z_response = ui.add(egui::DragValue::new(&mut euler_angles.z).prefix("z: "));
                if x_response.changed() || y_response.changed() || z_response.changed() {
                    self.rotation = Quat::from_euler(
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
                    egui::DragValue::new(&mut self.scale.x)
                        .prefix("x: ")
                        .speed(0.01),
                );
                ui.add(
                    egui::DragValue::new(&mut self.scale.y)
                        .prefix("y: ")
                        .speed(0.01),
                );
                ui.add(
                    egui::DragValue::new(&mut self.scale.z)
                        .prefix("z: ")
                        .speed(0.01),
                );
            });
        });
    }
}

impl<T: bevy::asset::Asset> DisplayableComponent for Handle<T> {
    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(self.clone());
    }
}

impl DisplayableComponent for GlobalTransform {
    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
    }
}

impl DisplayableComponent for Visibility {
    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
    }
}

impl DisplayableComponent for ComputedVisibility {
    fn clone_onto(&self, commands: &mut EntityCommands) {
        commands.insert(self.clone());
    }
}
