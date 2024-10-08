use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;

// mod components;
pub mod tesselate;
pub use crate::scene::tesselate::shapes;

pub mod inputs;
use crate::scene::inputs::components::*;
use crate::scene::inputs::resources::*;

use nalgebra::{Point3, Quaternion};

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
    mut window: Query<&mut Window>
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
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    });

    let window_instance = window.single_mut();
    let window_size = Vec2::new(window_instance.width(), window_instance.height());

    let static_scene_viewport_position = Vec2::new(0.2, 0.0);
    let static_scene_viewport_size = Vec2::new(1.0 - static_scene_viewport_position.x, 1.0 - static_scene_viewport_position.y);

    let scene_viewport_physical_size = UVec2::new((window_size.x * static_scene_viewport_size.x) as u32, (window_size.y * static_scene_viewport_size.y) as u32);
    let scene_viewport_physical_position = UVec2::new((window_size.x * static_scene_viewport_position.x) as u32, (window_size.y * static_scene_viewport_position.y) as u32);

    // Camera
    let initial_distance = 10.0;
    let target = Vec3::ZERO;
    let camera_transform = Transform::from_xyz(0.0, 5.0, 10.0).looking_at(target, Vec3::Y);
    
    commands.insert_resource(CameraTarget(Transform {
        translation: target,
        ..default()
    }));

    commands
        .spawn((
            Camera3dBundle {
                transform: camera_transform,
                camera: Camera {
                    viewport: Some(Viewport {
                        physical_size: scene_viewport_physical_size,
                        physical_position: scene_viewport_physical_position,
                        ..default()
                    }),
                    ..default()
                },
                ..default()
            },
            Main3DCameraMarker,
        ))
        .insert(OrbitCamera {
            target,
            distance: initial_distance,
            sensitivity: 0.1,
        });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::linear_rgba(red, green, blue, alpha), // Yellow color
            // emissive: LinearRgba{red, green, blue, alpha},
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 4.0, 1.0),
        ..default()
    });

    // Add some ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
