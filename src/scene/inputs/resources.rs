use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(pub Transform);

#[derive(Resource, Deref, DerefMut)]
pub struct CameraTarget(pub Transform);