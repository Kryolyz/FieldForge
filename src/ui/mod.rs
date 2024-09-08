use bevy::prelude::*;
pub mod resources;
use bevy::window::PrimaryWindow;
use nalgebra::Point3;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

use crate::scene::inputs::components::Main3DCameraMarker;
use crate::scene::inputs::resources::CameraTarget;
use crate::scene::tesselate::resources::PrimitiveType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_transform_system)
            .insert_resource(resources::Tree {
                root: resources::TreeNode {
                    name: "Root".to_string(),
                    collapsed: false,
                    children: vec![
                        resources::TreeNode {
                            name: "Child 1".to_string(),
                            primitive_type: PrimitiveType::Sphere,
                            ..default()
                        },
                        resources::TreeNode {
                            name: "Child 2".to_string(),
                            primitive_type: PrimitiveType::Sphere,
                            ..default()
                        },
                    ],
                    ..default()
                },
            });
    }
}

fn update_camera_transform_system(
    mut camera_query: Query<&mut Camera, With<Main3DCameraMarker>>,
    occupied_screen_space: Res<OccupiedScreenSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {

    // Check if the camera query finds a camera
    let mut camera = match camera_query.get_single_mut() {
        Ok(camera) => camera,
        Err(_) => {
            eprintln!("No camera found with Main3DCameraMarker");
            return;
        }
    };

    // Check if the window query finds a primary window
    let window = match windows.get_single() {
        Ok(window) => window,
        Err(_) => {
            eprintln!("No primary window found");
            return;
        }
    };

    let left_taken = occupied_screen_space.left; // / window.width();
    let right_taken = occupied_screen_space.right; // / window.width();
    let top_taken = occupied_screen_space.top; // / window.height();
    let bottom_taken = occupied_screen_space.bottom; // / window.height();

    let window = windows.single();
    let position_x = left_taken;
    let position_y = top_taken;

    if let Some(viewport) = &mut camera.viewport {
        viewport.physical_position.x = position_x as u32;
        viewport.physical_position.y = position_y as u32;
        viewport.physical_size.x = (window.width() - left_taken - right_taken) as u32;
        viewport.physical_size.y = (window.height() - top_taken - bottom_taken) as u32;
    }
}
