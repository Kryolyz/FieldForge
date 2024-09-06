// use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
pub mod shapes;
use nalgebra as na;
use nalgebra::Point3;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;

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

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  //   Create mesh from tessellation
  let sphere = shapes::UnitSphere::new();
  let mut mdc = tessellation::ManifoldDualContouring::new(&sphere, 0.2, 0.1);
  let triangles = mdc.tessellate().unwrap();

  let vertices_f32: Vec<[f32; 3]> = triangles
      .vertices
      .iter()
      .map(|&vertex| [vertex[0] as f32, vertex[1] as f32, vertex[2] as f32])
      .collect();

  let normals_f32: Vec<[f32; 3]> = vertices_f32
      .iter()
      .map(|&vertex_f32| {
          use tessellation::ImplicitFunction;
          let vertex_f64 = Point3::new(
              vertex_f32[0] as f64,
              vertex_f32[1] as f64,
              vertex_f32[2] as f64,
          );
          let normal_f64 = sphere.normal(&vertex_f64);
          [
              normal_f64.x as f32,
              normal_f64.y as f32,
              normal_f64.z as f32,
          ]
      })
      .collect();

  let faces: Vec<[u32; 3]> = triangles
      .faces
      .iter()
      .map(|&index| [index[0] as u32, index[1] as u32, index[2] as u32])
      .collect();

  let indices: Vec<u32> = faces.iter().flat_map(|&face| face).collect();

  // define triangle list first
  let mut mesh = Mesh::new(
      PrimitiveTopology::TriangleList,
      RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
  );
  mesh.insert_attribute(
      Mesh::ATTRIBUTE_POSITION,
      VertexAttributeValues::Float32x3(vertices_f32),
  );
  mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals_f32);
  mesh.insert_indices(Indices::U32(indices));
  // mesh.insert_attribute(Mesh::attribue_, values)

  // Sphere (yellow color)
  let red = 1.0;
  let green = 1.0;
  let blue = 0.0;
  let alpha = 1.0;

  commands.spawn(PbrBundle {
      mesh: meshes.add(mesh),
      material: materials.add(StandardMaterial {
          base_color: Color::linear_rgba(red, green, blue, alpha), // Yellow color
          ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
  });
}