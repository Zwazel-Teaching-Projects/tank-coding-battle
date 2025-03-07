use bevy::{prelude::*, utils::HashMap};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::game::game_state::{ClientState, FlagGameState, ProjectileState};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub tick: u64,
    #[serde(
        serialize_with = "serialize_hashmap",
        deserialize_with = "deserialize_hashmap"
    )]
    pub client_states: HashMap<Entity, Option<ClientState>>,
    #[serde(
        serialize_with = "serialize_hashmap",
        deserialize_with = "deserialize_hashmap"
    )]
    pub projectile_states: HashMap<Entity, ProjectileState>,
    #[serde(
        serialize_with = "serialize_hashmap",
        deserialize_with = "deserialize_hashmap"
    )]
    pub flag_states: HashMap<Entity, FlagGameState>,
}

fn serialize_hashmap<S, V>(map: &HashMap<Entity, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    V: Serialize,
{
    // Convert each Entity to its u64 id, then to a string.
    let string_map: HashMap<String, &V> = map
        .iter()
        .map(|(k, v)| (k.to_bits().to_string(), v))
        .collect();
    string_map.serialize(serializer)
}

fn deserialize_hashmap<'de, D, V>(deserializer: D) -> Result<HashMap<Entity, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    // Deserialize into a temporary HashMap with String keys.
    let string_map: HashMap<String, V> = HashMap::deserialize(deserializer)?;
    // Convert each string back to a u64, then to an Entity.
    string_map
        .into_iter()
        .map(|(k, v)| {
            k.parse::<u64>()
                .map(|num| (Entity::from_bits(num), v))
                .map_err(D::Error::custom)
        })
        .collect()
}
