use bevy::prelude::*;

#[derive(Debug, Clone, Reflect, Event)]
pub struct TeamScoredTrigger {
    pub scorer: Entity,
}

#[derive(Debug, Clone, Default, Reflect, Event)]
pub struct InitAllFlagsTrigger;

#[derive(Debug, Clone, Default, Reflect, Event)]
pub struct ResetFlagTrigger;

#[derive(Debug, Clone, Reflect, Event)]
pub struct FlagGotPickedUpTrigger {
    /// The entity that picked up the flag.
    /// Used to check if the flag was picked up by a player in the flag's team.
    /// If it was, the flag will be reset.
    /// If it wasn't, the flag will follow the player.
    pub carrier: Entity,
    pub flag: Entity,
}

#[derive(Debug, Clone, Reflect, Event)]
pub struct FlagGotDroppedTrigger {
    /// The entity that dropped the flag.
    pub carrier: Entity,
    /// The entity of the flag that was dropped.
    pub flag: Entity,
}
