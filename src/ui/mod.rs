use bevy::prelude::*;
mod resources;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

use crate::scene::inputs::components::Main3DCameraMarker;
use crate::scene::inputs::resources::CameraTarget;
use crate::scene::inputs::resources::OriginalCameraTransform;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::Tree {
            root: resources::TreeNode {
                name: "Root".to_string(),
                children: vec![
                    resources::TreeNode {
                        name: "Child 1".to_string(),
                        children: vec![],
                    },
                    resources::TreeNode {
                        name: "Child 2".to_string(),
                        children: vec![],
                    },
                ],
            },
        }).add_systems(Update, update_camera_transform_system);
    }
}

fn update_camera_transform_system(
    mut camera_query: Query<&mut Camera, With<Main3DCameraMarker>>,
    occupied_screen_space: Res<OccupiedScreenSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_target: Res<CameraTarget>,
) {
    let camera = &mut camera_query.single_mut();
    // let camera = camera_query.get_single_mut();

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
