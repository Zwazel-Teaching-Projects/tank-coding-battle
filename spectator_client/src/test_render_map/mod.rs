use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::GREEN,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use shared::{
    asset_handling::maps::{MapConfig, MapConfigSystemParam},
    main_state::MyMainState,
};

pub struct MyTestRenderMapPlugin;

impl Plugin for MyTestRenderMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapMeshMarker>()
            .add_systems(OnEnter(MyMainState::Ready), (create_map, spawn_camera))
            .add_systems(
                Update,
                (listen_for_map_changes,).run_if(in_state(MyMainState::Ready)),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(15.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Component, Default)]
#[reflect(Component)]
struct MapMeshMarker;

fn create_map(
    map_config: MapConfigSystemParam,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map_config = &map_config
        .get_map_config_from_name("test_map")
        .expect("Map not found")
        .map;

    let mesh = generate_mesh_from_grid(map_config.width, map_config.height, &map_config.tiles);
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(StandardMaterial {
        base_color: GREEN.into(),
        ..default()
    });

    commands.spawn((
        Name::new("GeneratedMapMesh"),
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        MapMeshMarker,
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn listen_for_map_changes(
    mut event: EventReader<AssetEvent<MapConfig>>,
    map_config: MapConfigSystemParam,
    mut map_mesh: Single<&mut Mesh3d, With<MapMeshMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in event.read() {
        match event {
            AssetEvent::Modified { id } => {
                info!("Map modified: {:?}", id);
                let map_config = &map_config
                    .get_map_config_from_asset_id(*id)
                    .expect("Map not found")
                    .map;

                let mesh =
                    generate_mesh_from_grid(map_config.width, map_config.height, &map_config.tiles);
                let mesh_handle = meshes.add(mesh);

                map_mesh.0 = mesh_handle;
            }
            _ => (),
        }
    }
}

pub fn generate_mesh_from_grid(width: usize, height: usize, grid: &Vec<Vec<f32>>) -> Mesh {
    let rows = height;
    let cols = width;

    info!("Ikit Claw commands: Generating centered cubes (with proper outward faces) from a grid of {}x{} cells", rows, cols);

    // Center the grid so that (0,0) is at its heart.
    let offset_x = (cols as f32) / 2.0;
    let offset_z = (rows as f32) / 2.0;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            let cell_top = grid[row][col];
            let cell_bottom = 0.0;
            let x = col as f32 - offset_x;
            let z = row as f32 - offset_z;

            // -- Top Face (normal: [0, 1, 0]) --
            {
                let base = positions.len() as u32;
                positions.push([x, cell_top, z]);
                positions.push([x + 1.0, cell_top, z]);
                positions.push([x + 1.0, cell_top, z + 1.0]);
                positions.push([x, cell_top, z + 1.0]);

                normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
                uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                // Reverse the winding order: [0,2,1] and [0,3,2]
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }

            // -- Bottom Face (normal: [0, -1, 0]) --
            {
                let base = positions.len() as u32;
                positions.push([x, cell_bottom, z + 1.0]);
                positions.push([x + 1.0, cell_bottom, z + 1.0]);
                positions.push([x + 1.0, cell_bottom, z]);
                positions.push([x, cell_bottom, z]);

                normals.extend_from_slice(&[[0.0, -1.0, 0.0]; 4]);
                uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }

            // -- Front Face (facing -Z, normal: [0, 0, -1]) --
            {
                let base = positions.len() as u32;
                positions.push([x, cell_bottom, z]); // bottom left
                positions.push([x + 1.0, cell_bottom, z]); // bottom right
                positions.push([x + 1.0, cell_top, z]); // top right
                positions.push([x, cell_top, z]); // top left

                normals.extend_from_slice(&[[0.0, 0.0, -1.0]; 4]);
                uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }

            // -- Back Face (facing +Z, normal: [0, 0, 1]) --
            {
                let base = positions.len() as u32;
                positions.push([x + 1.0, cell_bottom, z + 1.0]); // bottom left
                positions.push([x, cell_bottom, z + 1.0]); // bottom right
                positions.push([x, cell_top, z + 1.0]); // top right
                positions.push([x + 1.0, cell_top, z + 1.0]); // top left

                normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
                uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }

            // -- Left Face (facing -X, normal: [-1, 0, 0]) --
            {
                let base = positions.len() as u32;
                positions.push([x, cell_bottom, z + 1.0]); // bottom left
                positions.push([x, cell_bottom, z]); // bottom right
                positions.push([x, cell_top, z]); // top right
                positions.push([x, cell_top, z + 1.0]); // top left

                normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
                uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }

            // -- Right Face (facing +X, normal: [1, 0, 0]) --
            {
                let base = positions.len() as u32;
                positions.push([x + 1.0, cell_bottom, z]); // bottom left
                positions.push([x + 1.0, cell_bottom, z + 1.0]); // bottom right
                positions.push([x + 1.0, cell_top, z + 1.0]); // top right
                positions.push([x + 1.0, cell_top, z]); // top left

                normals.extend_from_slice(&[[1.0, 0.0, 0.0]; 4]);
                uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }
        }
    }

    let usage = if cfg!(debug_assertions) {
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    } else {
        RenderAssetUsages::RENDER_WORLD
    };
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, usage);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
