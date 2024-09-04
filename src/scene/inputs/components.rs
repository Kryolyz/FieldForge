use bevy::prelude::*;

#[derive(Component)]
pub struct OrbitCamera {
    pub target: Vec3,      // The point the camera orbits around
    pub distance: f32,     // The current distance from the target point
    pub sensitivity: f32,  // Sensitivity of the camera rotation
}

#[derive(Component)]
pub struct Main3DCameraMarker;