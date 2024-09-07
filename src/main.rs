use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
use strum::IntoEnumIterator;
mod resources;
mod scene;
mod ui;
use crate::ui::resources::{Tree, TreeNode};
use crate::ui::OccupiedScreenSpace;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::scene::tesselate::resources::PrimitiveType;

fn main() {
    // print!("Why would this not work?");
    // info!("{:?}", "PrimitiveType::None");
    // for prim in PrimitiveType::iter() {
    //     info!("{:?}", prim);
    //     print!("Please");
    // }

    // info!("{:?}", PrimitiveType::None);

    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
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
    let ctx = contexts.ctx_mut();

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
        ui.add_space(level as f32);
        ui.vertical(|ui| {
            ui.label(&node.name);

            // Dropdown menu for selecting primitive_type
            egui::ComboBox::from_label("Primitive Type")
                .selected_text(node.primitive_type.as_ref())
                .show_ui(ui, |ui| {
                    for primitive in PrimitiveType::iter() {
                        ui.selectable_value(
                            &mut node.primitive_type,
                            primitive,
                            primitive.as_ref(),
                        );
                    }
                });
        });
        ui.add(egui::widgets::Button::new("Add Node"));
    });

    for child in &mut node.children {
        display_tree(ui, child, level + 1);
    }
}
