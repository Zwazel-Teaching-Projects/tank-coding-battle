use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageTarget {
    #[default]
    Team,
    ServerOnly,
    All,
    Client, // TODO: we need to store the client ID here. what to use? Entity? SocketAddr? also, not send this out. because the receiver will just receive it, not send it.
}
