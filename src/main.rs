use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
mod resources;
mod scene;
mod ui;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::ui::OccupiedScreenSpace;

fn main() {
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
    mut is_last_selected: Local<bool>,
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            if ui
                .add(egui::widgets::Button::new("A button"))
                .clicked()
            {
                *is_last_selected = false;
            }
            if ui
                .add(egui::widgets::Button::new("Another button"))
                .clicked()
            {
                *is_last_selected = true;
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = 0.0;
    // egui::SidePanel::right("right_panel")
    //     .resizable(true)
    //     .show(ctx, |ui| {
    //         // ui.label("Right resizeable panel");
    //         ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    //     })
    //     .response
    //     .rect
    //     .width();
    occupied_screen_space.top = 0.0;
    // egui::TopBottomPanel::top("top_panel")
    //     .resizable(true)
    //     .show(ctx, |ui| {
    //         // ui.label("Top resizeable panel");
    //         ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    //     })
    //     .response
    //     .rect
    //     .height();
    occupied_screen_space.bottom = 0.0;
    // egui::TopBottomPanel::bottom("bottom_panel")
    //     .resizable(true)
    //     .show(ctx, |ui| {
    //         // ui.label("Bottom resizeable panel");
    //         ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    //     })
    //     .response
    //     .rect
    //     .height();
}
