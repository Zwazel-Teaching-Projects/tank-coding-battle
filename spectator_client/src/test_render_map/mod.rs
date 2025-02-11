use bevy::prelude::*;
use shared::{asset_handling::maps::MapConfigSystemParam, main_state::MyMainState};

pub struct MyTestRenderMapPlugin;

impl Plugin for MyTestRenderMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyMainState::Ready), create_map);
    }
}

fn create_map(map_config: MapConfigSystemParam) {
    let map_config = &map_config
        .get_map_config("test_map")
        .expect("Map not found")
        .map;

    println!("Map: {:?}", map_config);
}

/// Generates a mesh from a 2D height map. Each cell is a square on the XZ plane.
/// If a cell's height is a whole number, the floor is flat.
/// If it's a .5 value, the cell is rendered as a ramp rising northward.
pub fn generate_map_mesh(height_map: &Vec<Vec<f32>>) -> Mesh {
    let rows = height_map.len();
    if rows == 0 {
        return Mesh::new(PrimitiveTopology::TriangleList);
    }
    let cols = height_map[0].len();
    
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // For each cell in the height map:
    for z in 0..rows {
        for x in 0..cols {
            let h = height_map[z][x];
            let base = h.floor();
            // Determine if this cell is a ramp (i.e. its fractional part is non-zero)
            let is_ramp = (h - base).abs() > f32::EPSILON;

            // Define the four vertices for the cell.
            // The grid lies on the XZ plane, and Y is the height.
            // For flat cells, every vertex shares the same height.
            // For ramps, the south edge (at z) is lower (base) and the north edge (at z+1) is higher (base + 1).
            let (v0, v1, v2, v3) = if !is_ramp {
                (
                    [x as f32, h, z as f32],
                    [(x + 1) as f32, h, z as f32],
                    [(x + 1) as f32, h, (z + 1) as f32],
                    [x as f32, h, (z + 1) as f32],
                )
            } else {
                (
                    [x as f32, base, z as f32],               // South-west: low
                    [(x + 1) as f32, base, z as f32],           // South-east: low
                    [(x + 1) as f32, base + 1.0, (z + 1) as f32], // North-east: high
                    [x as f32, base + 1.0, (z + 1) as f32],       // North-west: high
                )
            };

            // Record the starting index for this cell's vertices.
            let start_idx = positions.len() as u32;
            positions.push(v0);
            positions.push(v1);
            positions.push(v2);
            positions.push(v3);

            // Compute normals.
            // For flat cells, the normal points straight upward.
            // For ramps, we compute an approximate normal from two edges.
            let normal = if !is_ramp {
                [0.0, 1.0, 0.0]
            } else {
                let edge1 = [
                    v1[0] - v0[0],
                    v1[1] - v0[1],
                    v1[2] - v0[2],
                ];
                let edge2 = [
                    v3[0] - v0[0],
                    v3[1] - v0[1],
                    v3[2] - v0[2],
                ];
                let cross = [
                    edge1[1] * edge2[2] - edge1[2] * edge2[1],
                    edge1[2] * edge2[0] - edge1[0] * edge2[2],
                    edge1[0] * edge2[1] - edge1[1] * edge2[0],
                ];
                let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
                if len != 0.0 { [cross[0] / len, cross[1] / len, cross[2] / len] } else { [0.0, 1.0, 0.0] }
            };
            normals.extend_from_slice(&[normal; 4]);

            // Simple UV mapping for the cell.
            uvs.push([0.0, 0.0]);
            uvs.push([1.0, 0.0]);
            uvs.push([1.0, 1.0]);
            uvs.push([0.0, 1.0]);

            // Two triangles for the cell: (v0, v1, v2) and (v0, v2, v3)
            indices.push(start_idx);
            indices.push(start_idx + 1);
            indices.push(start_idx + 2);
            indices.push(start_idx);
            indices.push(start_idx + 2);
            indices.push(start_idx + 3);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
