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

use crate::{
    main_state::MyMainState,
    networking::messages::message_data::message_error_types::ErrorMessageTypes,
};

pub struct MyMapPlugin;

impl Plugin for MyMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TeamConfig>()
            .register_type::<MapConfig>()
            .register_type::<MapDefinition>()
            .register_type::<TileDefinition>()
            .register_type::<LayerDefinition>()
            .register_type::<LayerType>()
            .register_type::<MarkerDefinition>()
            .register_type::<MarkerType>()
            .register_type::<SimplifiedRGB>()
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

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize, PartialEq)]
pub struct MapConfig {
    pub teams: HashMap<String, TeamConfig>,
    pub map: MapDefinition,
}

impl MapConfig {
    pub fn insert_player_into_team(
        &mut self,
        team_name: &str,
        player: Entity,
    ) -> Result<(), ErrorMessageTypes> {
        match self.teams.get_mut(team_name) {
            Some(team) => {
                if team.players.len() < team.max_players {
                    team.players.push(player);
                    Ok(())
                } else {
                    Err(ErrorMessageTypes::TeamFull(format!(
                        "Team {} is full",
                        team_name
                    )))
                }
            }
            None => Err(ErrorMessageTypes::TeamDoesNotExist(format!(
                "Team {} not found",
                team_name
            ))),
        }
    }

    pub fn remove_player_from_team(&mut self, player: Entity) {
        for team in self.teams.values_mut() {
            team.players.retain(|&x| x != player);
        }
    }

    pub fn get_team(&self, team_name: &str) -> Option<&TeamConfig> {
        self.teams.get(team_name)
    }

    pub fn get_team_of_player(&self, player: Entity) -> Option<(String, &TeamConfig)> {
        for (team_name, team) in self.teams.iter() {
            if team.players.contains(&player) {
                return Some((team_name.clone(), team));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamConfig {
    pub color: SimplifiedRGB,
    pub max_players: usize,

    #[serde(skip)]
    pub players: Vec<Entity>,
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

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapDefinition {
    pub width: usize,
    pub height: usize,

    pub floor_color: SimplifiedRGB,

    /// A 2D array of height values—row by row.
    /// For a grid of size `height x width`, we'll have `height` sub-vectors,
    /// each containing `width` floats.
    pub tiles: Vec<Vec<f32>>,

    pub layers: Vec<LayerDefinition>,

    pub markers: Vec<MarkerDefinition>,
}

impl MapDefinition {
    pub fn get_height_at(&self, x: usize, y: usize) -> f32 {
        self.tiles[y][x]
    }

    pub fn get_real_world_position(&self, x: usize, y: usize) -> Vec3 {
        Vec3::new(x as f32 + 0.5, self.get_height_at(x, y), y as f32 + 0.5)
    }

    pub fn get_all_spawn_points_of_group(&self, group: &str) -> Vec<(Vec3, usize)> {
        self.markers
            .iter()
            .filter_map(|marker| {
                if marker.group == group {
                    match &marker.kind {
                        MarkerType::Spawn { spawn_number } => Some((
                            self.get_real_world_position(marker.tile.x, marker.tile.y),
                            *spawn_number,
                        )),
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
                    MarkerType::Spawn { spawn_number: n } if *n == spawn_number => {
                        Some(self.get_real_world_position(marker.tile.x, marker.tile.y))
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TileDefinition {
    pub x: usize,
    pub y: usize,
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
    },
    Flag,
}

impl Default for MarkerType {
    fn default() -> Self {
        MarkerType::Spawn { spawn_number: 0 }
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
