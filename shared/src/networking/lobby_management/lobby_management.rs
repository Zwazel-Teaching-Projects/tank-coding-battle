use bevy::prelude::*;

use crate::game::player_handling::BotState;

use super::InTeam;

#[derive(Debug, Default, Clone)]
pub struct LobbyManagementArgument {
    pub sender: Option<Entity>,
    pub target_player: Option<Entity>,
}

pub struct LobbyManagementSystemParam<'w, 's> {
    pub bots: Query<'w, 's, (Entity, &'static InTeam, &'static BotState)>,
}

impl<'w, 's> LobbyManagementSystemParam<'w, 's> {
    pub fn get_sender_state(&self, arg: &LobbyManagementArgument) -> Result<BotState, String> {
        if let Some(sender) = arg.sender {
            if let Ok((_, _, state)) = self.bots.get(sender) {
                Ok(state.clone())
            } else {
                Err("Sender is not a bot".to_string())
            }
        } else {
            Err("Sender is not set".to_string())
        }
    }

    pub fn targets_get_self(&self, arg: LobbyManagementArgument) -> Result<Vec<Entity>, String> {
        if let Some(sender) = arg.sender {
            if let Ok(_) = self.bots.get(sender) {
                Ok(vec![sender])
            } else {
                Err("Sender is not a bot".to_string())
            }
        } else {
            Err("Sender is not set".to_string())
        }
    }

    pub fn targets_get_single_player(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        if let Some(target_player) = arg.target_player {
            if let Ok(_) = self.bots.get(target_player) {
                Ok(vec![target_player])
            } else {
                Err("Target player is not a bot".to_string())
            }
        } else {
            Err("Target player is not set".to_string())
        }
    }

    /// Returns all players in the game, excluding the sender itself.
    pub fn targets_get_everyone_in_game(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        if let Some(sender) = arg.sender {
            let mut players_in_game = Vec::new();
            for (entity, _, _) in self.bots.iter() {
                if entity != sender {
                    players_in_game.push(entity);
                }
            }
            Ok(players_in_game)
        } else {
            Err("Sender is not set".to_string())
        }
    }

    pub fn targets_get_empty(&self, _arg: LobbyManagementArgument) -> Result<Vec<Entity>, String> {
        Ok(vec![])
    }

    /// Returns all players in the same team as the sender. Excludes the sender itself.
    pub fn targets_get_players_in_team(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        if let Some(sender) = arg.sender {
            if let Ok((_, sender_team, _)) = self.bots.get(sender) {
                let mut players_in_team = Vec::new();
                for (entity, team, _) in self.bots.iter() {
                    if entity != sender && team == sender_team {
                        players_in_team.push(entity);
                    }
                }
                Ok(players_in_team)
            } else {
                Err("Sender is not a bot".to_string())
            }
        } else {
            Err("Sender is not set".to_string())
        }
    }
}
