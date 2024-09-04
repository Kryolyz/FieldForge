// use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
pub mod shapes;
use nalgebra as na;

impl tessellation::ImplicitFunction<f64> for shapes::UnitSphere {
  fn bbox(&self) -> &tessellation::BoundingBox<f64> {
      &self.bbox
  }
  fn value(&self, p: &na::Point3<f64>) -> f64 {
      return na::Vector3::new(p.x, p.y, p.z).norm() - 1.0;
  }
  fn normal(&self, p: &na::Point3<f64>) -> na::Vector3<f64> {
      return na::Vector3::new(p.x, p.y, p.z).normalize();
  }
}