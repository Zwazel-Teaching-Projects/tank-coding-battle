use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
    mapped::AssetFileStem,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::main_state::MyMainState;

pub struct MyMapPlugin;

impl Plugin for MyMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapConfig>()
            .register_type::<MapDefinition>()
            .register_type::<TileDefinition>()
            .register_type::<LayerDefinition>()
            .register_type::<LayerType>()
            .register_type::<MarkerDefinition>()
            .register_type::<MarkerType>()
            .register_type::<SimplifiedRGB>()
            .register_type::<LookDirection>()
            .register_type::<TileNeighbours>()
            .register_type::<TeamInMapConfig>()
            .register_type::<PremadePartialMapDefinition>()
            .register_type::<PartialMapDefinition>()
            .configure_loading_state(
                LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<AllMapsAsset>(),
            )
            .add_plugins(RonAssetPlugin::<MapConfig>::new(&["map.ron"]));
    }
}

#[derive(Debug, Default, Clone, AssetCollection, Resource)]
pub struct AllMapsAsset {
    #[asset(path = "maps", collection(mapped, typed))]
    pub maps: HashMap<AssetFileStem, Handle<MapConfig>>,
}

#[derive(Debug, Reflect, Clone, Asset, Deserialize, PartialEq)]
pub struct MapConfig {
    pub teams: TeamInMapConfig,
    pub map: MapDefinition,
}

/// Defines how many teams and how many players per team are to be expected in this map
#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamInMapConfig {
    pub team_count: usize,
    pub players_per_team: usize,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<(f32, f32, f32)> for SimplifiedRGB {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        SimplifiedRGB { r, g, b }
    }
}

impl From<Color> for SimplifiedRGB {
    fn from(color: Color) -> Self {
        let color = color.to_linear();
        SimplifiedRGB {
            r: color.red,
            g: color.green,
            b: color.blue,
        }
    }
}

impl From<SimplifiedRGB> for Color {
    fn from(SimplifiedRGB { r, g, b }: SimplifiedRGB) -> Self {
        Color::linear_rgba(r, g, b, 1.0)
    }
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum MapDefinition {
    PremadeMap {
        name: String,
        map: PartialMapDefinition,
    },
    Custom {
        middle_part: PremadePartialMapDefinition,
        other_parts: Vec<PartialMapDefinition>,
        /// Will be setup together with the other parts and the middle part
        #[serde(skip)]
        full_map: Option<PartialMapDefinition>,
    },
}

impl MapDefinition {
    /// If custom, merge all parts into one map
    pub fn setup_map(&mut self) {
        match self {
            MapDefinition::PremadeMap { .. } => {}
            MapDefinition::Custom {
                middle_part,
                other_parts,
                ..
            } => {
                // check how many other parts there are. for now, expect exactly 2. put one in front, then middle, then last
                if other_parts.len() == 2 {
                    let mut full_map = middle_part.map.clone();
                    let first_part = &other_parts[0];
                    let last_part = &other_parts[1];

                    // put first part in front
                    full_map.tiles = first_part.tiles.clone();
                    full_map.layers = first_part.layers.clone();
                    full_map.markers = first_part.markers.clone();

                    // put last part at the end
                    full_map.tiles.extend(last_part.tiles.clone());
                    full_map.layers.extend(last_part.layers.clone());
                    full_map.markers.extend(last_part.markers.clone());

                    *self = MapDefinition::Custom {
                        middle_part: middle_part.clone(),
                        other_parts: other_parts.clone(),
                        full_map: Some(full_map),
                    };
                } else {
                    error!(
                        "Expected exactly 2 other parts for custom map, got {}",
                        other_parts.len()
                    );
                }
            }
        }
    }

    pub fn get_floor_height_of_tile(&self, tile: impl Into<TileDefinition>) -> Option<f32> {
        let tile = TileDefinition::from(tile.into());
        let tiles = match self {
            MapDefinition::PremadeMap { map, .. } => &map.tiles,
            MapDefinition::Custom { full_map, .. } => full_map.as_ref()?.tiles.as_ref(),
        };

        tiles.get(tile.y).and_then(|row| row.get(tile.x)).copied()
    }

    pub fn get_real_world_position_of_tile(&self, tile: impl Into<TileDefinition>) -> Option<Vec3> {
        self.get_center_of_tile(tile.into())
    }

    pub fn grid_in_real_world(&self) -> Vec<Vec3> {
        let map = match self {
            MapDefinition::PremadeMap { map, .. } => map,
            MapDefinition::Custom { full_map, .. } => full_map.as_ref().unwrap(),
        };

        let mut grid = Vec::new();
        for y in 0..map.depth {
            for x in 0..map.width {
                if let Some(pos) = self.get_real_world_position_of_tile((x, y)) {
                    grid.push(pos);
                }
            }
        }
        grid
    }

    pub fn get_all_spawn_points_of_group(&self, group: &str) -> Vec<(Vec3, usize)> {
        self.markers
            .iter()
            .filter_map(|marker| {
                if marker.group == group {
                    match &marker.kind {
                        MarkerType::Spawn { spawn_number, .. } => self
                            .get_real_world_position_of_tile((marker.tile.x, marker.tile.y))
                            .map(|pos| (pos, *spawn_number)),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_spawn_point_position(&self, group: &str, spawn_number: usize) -> Option<Vec3> {
        self.markers.iter().find_map(|marker| {
            if marker.group == group {
                match &marker.kind {
                    MarkerType::Spawn {
                        spawn_number: n, ..
                    } if *n == spawn_number => {
                        Some(self.get_real_world_position_of_tile((marker.tile.x, marker.tile.y))?)
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    pub fn get_spawn_point_rotation(&self, group: &str, spawn_number: usize) -> Option<Quat> {
        self.markers.iter().find_map(|marker| {
            if marker.group == group {
                match &marker.kind {
                    MarkerType::Spawn {
                        spawn_number: n,
                        look_direction,
                    } if *n == spawn_number => Some(look_direction.to_quat()),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    pub fn get_closest_tile(&self, real_world_position: Vec3) -> Option<TileDefinition> {
        if real_world_position.y < 0.0 {
            return None;
        }

        const CELL_SIZE: f32 = 1.0;
        let (real_world_x, real_world_z) = (real_world_position.x, real_world_position.z);

        let real_world_grid = self.grid_in_real_world();
        let mut closest_tile = None;

        for (i, real_world_tile_pos) in real_world_grid.iter().enumerate() {
            let distance = (real_world_tile_pos.x - real_world_x).abs()
                + (real_world_tile_pos.z - real_world_z).abs();
            if distance < CELL_SIZE / 2.0 {
                return Some(TileDefinition {
                    x: i % self.width,
                    y: i / self.width,
                });
            }

            if let Some((_, closest_distance)) = closest_tile {
                if distance < closest_distance {
                    closest_tile = Some((i, distance));
                }
            } else {
                closest_tile = Some((i, distance));
            }
        }

        closest_tile.map(|(i, _)| TileDefinition {
            x: i % self.width,
            y: i / self.width,
        })
    }

    pub fn get_neighbours(&self, tile: impl Into<TileDefinition>) -> TileNeighbours {
        let TileDefinition { x, y } = tile.into();
        let center = TileDefinition { x, y };
        let north = (y + 1 < self.depth).then(|| TileDefinition { x, y: y + 1 });
        let south = (y > 0).then(|| TileDefinition { x, y: y - 1 });
        let east = (x > 0).then(|| TileDefinition { x: x - 1, y });
        let west = ((x + 1) < self.width).then(|| TileDefinition { x: x + 1, y });

        let north_east = if x > 0 && (y + 1) < self.depth {
            Some(TileDefinition { x: x - 1, y: y + 1 })
        } else {
            None
        };
        let north_west = if (x + 1) < self.width && (y + 1) < self.depth {
            Some(TileDefinition { x: x + 1, y: y + 1 })
        } else {
            None
        };
        let south_east = if x > 0 && y > 0 {
            Some(TileDefinition { x: x - 1, y: y - 1 })
        } else {
            None
        };
        let south_west = if (x + 1) < self.width && y > 0 {
            Some(TileDefinition { x: x + 1, y: y - 1 })
        } else {
            None
        };

        TileNeighbours {
            center,
            north,
            east,
            south,
            west,
            north_east,
            north_west,
            south_east,
            south_west,
        }
    }

    pub fn get_center_of_map(&self) -> Vec3 {
        Vec3::new(self.width as f32 / 2.0, 0.0, self.depth as f32 / 2.0)
    }

    pub fn get_highest_point(&self) -> f32 {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .fold(f32::MIN, f32::max)
    }

    pub fn get_center_of_tile(&self, tile: impl Into<TileDefinition>) -> Option<Vec3> {
        let TileDefinition { x, y } = tile.into();
        if let Some(height) = self.get_floor_height_of_tile((x, y)) {
            return Some(Vec3::new(x as f32 + 0.5, height, y as f32 + 0.5));
        }

        None
    }

    pub fn is_inside_bounds(&self, position: Vec3) -> bool {
        self.get_closest_tile(position).is_some()
    }
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PremadePartialMapDefinition {
    name: String,
    map: PartialMapDefinition,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PartialMapDefinition {
    pub width: usize,
    pub depth: usize,
    pub tiles: Vec<Vec<f32>>,
    pub layers: Vec<LayerDefinition>,
    pub markers: Vec<MarkerDefinition>,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TileDefinition {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for TileDefinition {
    fn from((x, y): (usize, usize)) -> Self {
        TileDefinition { x, y }
    }
}

impl From<(i32, i32)> for TileDefinition {
    fn from((x, y): (i32, i32)) -> Self {
        TileDefinition {
            x: x as usize,
            y: y as usize,
        }
    }
}

impl From<TileDefinition> for (usize, usize) {
    fn from(TileDefinition { x, y }: TileDefinition) -> Self {
        (x, y)
    }
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
pub struct TileNeighbours {
    pub center: TileDefinition,
    pub north: Option<TileDefinition>,
    pub east: Option<TileDefinition>,
    pub south: Option<TileDefinition>,
    pub west: Option<TileDefinition>,
    pub north_east: Option<TileDefinition>,
    pub north_west: Option<TileDefinition>,
    pub south_east: Option<TileDefinition>,
    pub south_west: Option<TileDefinition>,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayerDefinition {
    pub kind: LayerType,
    /// A cost modifier for pathfinding or movement / Maybe also use this to slow down?
    pub cost_modifier: f32,
    // TODO: Add a hide modifier?
    /// A list of (x, y) coordinates for cells that belong to this layer
    pub tiles: Vec<TileDefinition>,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayerType {
    #[default]
    Forest,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDefinition {
    pub tile: TileDefinition,
    /// The group this marker belongs to. for example a team
    pub group: String,

    pub kind: MarkerType,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum MarkerType {
    #[serde(rename_all = "camelCase")]
    Spawn {
        spawn_number: usize,
        look_direction: LookDirection,
    },
    #[serde(rename_all = "camelCase")]
    FlagBase { flag_number: usize },
}

impl Default for MarkerType {
    fn default() -> Self {
        MarkerType::Spawn {
            spawn_number: 0,
            look_direction: LookDirection::default(),
        }
    }
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LookDirection {
    #[default]
    North,
    East,
    South,
    West,
}

impl LookDirection {
    pub fn to_quat(&self) -> Quat {
        match self {
            LookDirection::North => Quat::from_rotation_y(0.0),
            LookDirection::East => Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
            LookDirection::South => Quat::from_rotation_y(std::f32::consts::PI),
            LookDirection::West => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        }
    }

    pub fn inverse(&self) -> LookDirection {
        match self {
            LookDirection::North => LookDirection::South,
            LookDirection::East => LookDirection::West,
            LookDirection::South => LookDirection::North,
            LookDirection::West => LookDirection::East,
        }
    }
}

#[derive(SystemParam)]
pub struct MapConfigSystemParam<'w> {
    maps_asset: Res<'w, AllMapsAsset>,
    map_configs: Res<'w, Assets<MapConfig>>,
}

impl<'w> MapConfigSystemParam<'w> {
    pub fn get_map_config_from_name(&self, map_name: &str) -> Option<&MapConfig> {
        let map_name = if map_name.ends_with(".map") {
            map_name.to_string()
        } else {
            format!("{}.map", map_name)
        };

        self.maps_asset
            .maps
            .iter()
            .find(|(stem, _)| stem.as_ref() == map_name)
            .and_then(|(_, handle)| self.map_configs.get(handle))
    }

    pub fn get_map_config_from_asset_id(&self, asset_id: AssetId<MapConfig>) -> Option<&MapConfig> {
        self.map_configs.get(asset_id)
    }

    pub fn list_map_names(&self) -> Vec<String> {
        self.maps_asset
            .maps
            .iter()
            .map(|(stem, _)| stem.as_ref().to_string().replace(".map", ""))
            .collect()
    }
}
