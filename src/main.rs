// use bevy::{
//     pbr::wireframe::{Wireframe, WireframePlugin},
//     prelude::*,
//     render::{
//         mesh::Indices,
//         options::WgpuOptions,
//         render_resource::{PrimitiveTopology, WgpuFeatures},
//     },
// };

use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        pipeline::PrimitiveTopology,
        wireframe::{Wireframe, WireframePlugin},
    },
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};

fn main() {
    // App::new()
    App::build()
        .insert_resource(WgpuOptions {
            // features: WgpuFeatures::POLYGON_MODE_LINE,
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mesh_materials: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // custom mesh square
    commands
        .spawn_bundle(PbrBundle {
            mesh: create_mesh(mesh_materials),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Wireframe);
    // light
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..Default::default()
    // });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn create_mesh(mut mesh_materials: ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let vertex_count = 4usize; // 4 vertices (2 triangles, 4 corners of a square)

    vertices.resize(vertex_count, [0.0f32, 0.0f32, 0.0f32]);
    normals.resize(vertex_count, [0.0f32, 1.0f32, 0.0f32]);
    let uvs = vec![[0.0, 0.0, 0.0]; vertices.len()];

    // vertex
    let mut vertex_index = 0;
    for cy in 0..2 {
        // top (0) to bottom (1)
        for cx in 0..2 {
            // left (0) to right (1)
            vertices[vertex_index] = [cx as f32, 0., cy as f32];
            vertex_index += 1;
        }
    }

    // index
    indices.extend(
        [
            0, // top left      \---
            3, // bottom right   \ |
            1, // top right       \|
        ]
        .iter(),
    );
    indices.extend(
        [
            0, // top left      |\
            2, // bottom left   | \
            3, // bottom right  ---\
        ]
        .iter(),
    );

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh_materials.add(mesh)
}
