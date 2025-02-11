use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::GREEN,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use shared::{asset_handling::maps::MapConfigSystemParam, main_state::MyMainState};

pub struct MyTestRenderMapPlugin;

impl Plugin for MyTestRenderMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyMainState::Ready), (create_map, spawn_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 10.0),
    ));
}

fn create_map(
    map_config: MapConfigSystemParam,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map_config = &map_config
        .get_map_config("test_map")
        .expect("Map not found")
        .map;

    let mesh = generate_mesh_from_grid(&map_config.tiles);
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(StandardMaterial {
        base_color: GREEN.into(),
        ..default()
    });

    commands.spawn((
        Name::new("GeneratedMapMesh"),
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
    ));
}

/// Generates a mesh from a 2D grid of heights.
/// Each grid cell becomes a quad (two triangles) with its four vertices at the cell's height.
/// The grid is assumed to be organized as rows, where each cell is a square of unit size.
pub fn generate_mesh_from_grid(grid: &Vec<Vec<f32>>) -> Mesh {
    let rows = grid.len();
    if rows == 0 {
        return Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
    }
    let cols = grid[0].len();

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // For each cell in the grid, create a quad (4 vertices, 2 triangles)
    for row in 0..rows {
        for col in 0..cols {
            let height = grid[row][col];
            // Define the four corners of the cell.
            // Note: We use col as the X coordinate and row as the Z coordinate.
            let bottom_left = [col as f32, height, row as f32];
            let bottom_right = [(col + 1) as f32, height, row as f32];
            let top_right = [(col + 1) as f32, height, (row + 1) as f32];
            let top_left = [col as f32, height, (row + 1) as f32];

            // Save current vertex start index.
            let base_index = positions.len() as u32;

            // Push vertices for the current quad.
            positions.push(bottom_left);
            positions.push(bottom_right);
            positions.push(top_right);
            positions.push(top_left);

            // All vertices share an upward normal.
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);

            // Assign UV coordinates for the quad.
            uvs.push([0.0, 0.0]);
            uvs.push([1.0, 0.0]);
            uvs.push([1.0, 1.0]);
            uvs.push([0.0, 1.0]);

            // Two triangles: (0, 1, 2) and (0, 2, 3)
            indices.push(base_index);
            indices.push(base_index + 1);
            indices.push(base_index + 2);
            indices.push(base_index);
            indices.push(base_index + 2);
            indices.push(base_index + 3);
        }
    }

    // Build the mesh with the positions, normals, uvs, and indices.
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
