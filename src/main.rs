use std::hash::Hash;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
use bevy_egui::egui::{Ui, Widget};
mod resources;
mod scene;
mod ui;
use crate::ui::resources::{Tree, TreeNode};
use crate::ui::OccupiedScreenSpace;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSet};

use crate::scene::tesselate::resources::PrimitiveType;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<OccupiedScreenSpace>()
        .add_plugins(scene::ScenePlugin)
        .add_plugins(ui::UiPlugin)
        .add_systems(Update, scene::inputs::camera_orbit_controls)
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut distance_field_tree: ResMut<Tree>,
) {
    // let ctx = contexts.try_ctx_mut();
    let ctx = match contexts.try_ctx_mut() {
        Some(ctx) => ctx,
        None => {
            eprintln!("EguiContexts is uninitialized. Expected during shutdown.");
            return;
        }
    };

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Object Tree");

            // Display the tree structure
            display_tree(ui, &mut distance_field_tree.root, 0);

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.right = 0.0;
    occupied_screen_space.top = 0.0;
    occupied_screen_space.bottom = 0.0;
}

fn display_tree(ui: &mut egui::Ui, node: &mut TreeNode, level: u32) {
    ui.horizontal(|ui| {
        ui.add_space(level as f32 * 10.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let arrow = if node.collapsed { "▶" } else { "▼" };
                if ui.button(arrow).clicked() {
                    node.collapsed = !node.collapsed;
                }
                ui.label("Name:");
                ui.text_edit_singleline(&mut node.name);
            });

            if !node.collapsed {
                // Delete button
                if ui.button("Delete").clicked() {
                    node.marked_for_deletion = true;
                }
                // Dropdown menu for selecting primitive_type
                egui::ComboBox::from_id_source(ui.id().with(&node))
                    .selected_text(format!("{:?}", node.primitive_type))
                    .show_ui(ui, |ui| {
                        for primitive in PrimitiveType::iter() {
                            ui.selectable_value(
                                &mut node.primitive_type,
                                primitive,
                                format!("{:?}", primitive),
                            );
                        }
                    });

                // Editable text fields for translation
                ui.horizontal(|ui| {
                    ui.label("Translation:");
                    ui.add(
                        egui::DragValue::new(&mut node.transform.translation.x)
                            .speed(0.1)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut node.transform.translation.y)
                            .speed(0.1)
                            .prefix("y: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut node.transform.translation.z)
                            .speed(0.1)
                            .prefix("z: "),
                    );
                });

                // Convert quaternion to Euler angles
                let (mut yaw, mut pitch, mut roll) =
                    node.transform.rotation.to_euler(EulerRot::YXZ);

                // Editable text fields for rotation (Euler angles)
                ui.horizontal(|ui| {
                    ui.label("Rotation:");
                    ui.add(egui::DragValue::new(&mut yaw).speed(0.1).prefix("Yaw: "));
                    ui.add(
                        egui::DragValue::new(&mut pitch)
                            .speed(0.1)
                            .prefix("Pitch: "),
                    );
                    ui.add(egui::DragValue::new(&mut roll).speed(0.1).prefix("Roll: "));
                });

                // Update the quaternion based on edited Euler angles
                node.transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

                let add_node_button = add_button(ui, node, "Add Node");
                if add_node_button.clicked() {
                    let mut new_node = TreeNode::default();
                    new_node.name = "New Child".to_string();
                    info!("{:?}", new_node.name);
                    node.add_child(new_node);
                };
            }

            let separator = ui.add(egui::Separator::default());

            // separator.
        });
    });

    if !node.collapsed {
        node.children.retain(|child| !child.marked_for_deletion);
        for child in &mut node.children {
            display_tree(ui, child, level + 1);
        }
    }
}

fn add_button(ui: &mut Ui, node: &mut TreeNode, name: &str) -> egui::Response {
    let button_id = ui.id().with(&node).with(name);
    let mut button = ui.add(egui::widgets::Button::new(name));
    button.id = button_id;
    return button;
}
