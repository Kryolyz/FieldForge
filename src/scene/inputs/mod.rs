use bevy::prelude::*;
use bevy::input::mouse::*;

pub mod components;


pub fn camera_orbit_controls(
  mut query: Query<(&mut Transform, &mut components::OrbitCamera), With<components::Main3DCameraMarker>>,
  mut mouse_motion_events: EventReader<MouseMotion>,
  mut mouse_wheel_events: EventReader<MouseWheel>,
  mouse_button_input: Res<ButtonInput<MouseButton>>, // Add this line to correctly reference the Input resource
  keys: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  // Constants
  let zoom_speed = 1.0;
  let pan_speed = 0.5;

  // if the spacebar is pressed, reset the camera to how it was initialized
  if keys.just_pressed(KeyCode::Space) {
      for (mut transform, mut orbit_camera) in query.iter_mut() {
          transform.translation = Vec3::new(0.0, 5.0, 10.0);
          transform.look_at(Vec3::ZERO, Vec3::Y);
          orbit_camera.distance = 10.0;
          orbit_camera.target = Vec3::ZERO;
      }
  }

  for (mut transform, mut orbit_camera) in query.iter_mut() {
      if mouse_button_input.pressed(MouseButton::Right) { 
          for event in mouse_motion_events.read() {
              debug!("{:?}", event); let delta = event.delta; 
              let yaw = orbit_camera.sensitivity * delta.x * time.delta_seconds(); 
              let pitch = orbit_camera.sensitivity * delta.y * time.delta_seconds(); 
              // Calculate the right and up vectors of the camera 
              let right = transform.rotation * Vec3::X; 
              let up = transform.rotation * Vec3::Y; // Apply rotation around target using the camera's local axes 
              transform.rotate_around( orbit_camera.target, Quat::from_axis_angle(up, -yaw) * Quat::from_axis_angle(right, -pitch), ); 
          } 
      }

      // Zoom
      for event in mouse_wheel_events.read() {
          debug!("{:?}", event);
          orbit_camera.distance -= event.y * zoom_speed;
          orbit_camera.distance = orbit_camera.distance.max(2.0).min(50.0); // Clamp the distance

          let direction = (transform.translation - orbit_camera.target).normalize();
          transform.translation = orbit_camera.target + direction * orbit_camera.distance;
      }

      // Panning
      if mouse_button_input.pressed(MouseButton::Middle) {
          for event in mouse_motion_events.read() {
              debug!("{:?}", event);
              let delta = event.delta;
              let right = transform.rotation * Vec3::X * -delta.x * pan_speed * time.delta_seconds();
              let up = transform.rotation * Vec3::Y * delta.y * pan_speed * time.delta_seconds();

              let pan_offset = right + up;
              orbit_camera.target += pan_offset;
              transform.translation += pan_offset;
          }
      }

      // Update the camera to always look at the target
      transform.look_at(orbit_camera.target, Vec3::Y);
  }
}