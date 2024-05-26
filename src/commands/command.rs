//! Wrapper for Company of Heroes 3 player commands.

use crate::commands::build_global_upgrade::from_data as build_global_upgrade_from_data;
use crate::commands::build_squad::from_data as build_squad_from_data;
use crate::commands::cancel_construction::from_data as cancel_construction_from_data;
use crate::commands::cancel_production::from_data as cancel_production_from_data;
use crate::commands::select_battlegroup::from_data as select_battlegroup_from_data;
use crate::commands::select_battlegroup_ability::from_data as select_battlegroup_ability_from_data;
use crate::commands::unknown::from_data as unknown_from_data;
use crate::commands::use_ability::from_data as use_ability_from_data;
use crate::commands::use_battlegroup_ability::from_data as use_battlegroup_ability_from_data;
use crate::commands::{
    BuildGlobalUpgrade as BuildGlobalUpgradeCommand, BuildSquad as BuildSquadCommand,
    CancelConstruction as CancelConstructionCommand, CancelProduction as CancelProductionCommand,
    SelectBattlegroup as SelectBattlegroupCommand,
    SelectBattlegroupAbility as SelectBattlegroupAbilityCommand, Unknown as UnknownCommand,
    UseAbility as UseAbilityCommand, UseBattlegroupAbility as UseBattlegroupAbilityCommand,
};
use crate::data::commands::CommandData;
use crate::data::ticks::Tick;
use crate::Command::{
    BuildGlobalUpgrade, BuildSquad, CancelConstruction, CancelProduction, SelectBattlegroup,
    SelectBattlegroupAbility, Unknown, UseAbility, UseBattlegroupAbility,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Wrapper for one of many Company of Heroes 3 player commands parsed from a replay file. For
/// details on the specifics of a given command, see the specific enum variants.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Command"))]
pub enum Command {
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::BuildGlobalUpgradeCommand")
    )]
    BuildGlobalUpgrade(BuildGlobalUpgradeCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::BuildSquadCommand")
    )]
    BuildSquad(BuildSquadCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::CancelConstructionCommand")
    )]
    CancelConstruction(CancelConstructionCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::CancelProductionCommand")
    )]
    CancelProduction(CancelProductionCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::SelectBattlegroupCommand")
    )]
    SelectBattlegroup(SelectBattlegroupCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::SelectBattlegroupAbilityCommand")
    )]
    SelectBattlegroupAbility(SelectBattlegroupAbilityCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::UseAbilityCommand")
    )]
    UseAbility(UseAbilityCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::UseBattlegroupAbilityCommand")
    )]
    UseBattlegroupAbility(UseBattlegroupAbilityCommand),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::UnknownCommand")
    )]
    Unknown(UnknownCommand),
}

impl Command {
    #[cfg(feature = "magnus")]
    pub fn extract_build_global_upgrade(&self) -> BuildGlobalUpgradeCommand {
        if let BuildGlobalUpgrade(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_build_squad(&self) -> BuildSquadCommand {
        if let BuildSquad(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_cancel_construction(&self) -> CancelConstructionCommand {
        if let CancelConstruction(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_cancel_production(&self) -> CancelProductionCommand {
        if let CancelProduction(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_select_battlegroup(&self) -> SelectBattlegroupCommand {
        if let SelectBattlegroup(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_select_battlegroup_ability(&self) -> SelectBattlegroupAbilityCommand {
        if let SelectBattlegroupAbility(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_use_ability(&self) -> UseAbilityCommand {
        if let UseAbility(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_use_battlegroup_ability(&self) -> UseBattlegroupAbilityCommand {
        if let UseBattlegroupAbility(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_unknown(&self) -> UnknownCommand {
        if let Unknown(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }
}

pub(crate) fn command_from_data(data: &CommandData, tick: i32) -> (u8, Command) {
    match data {
        CommandData::BuildGlobalUpgrade(build_global_upgrade) => (
            build_global_upgrade.player_id,
            BuildGlobalUpgrade(build_global_upgrade_from_data(build_global_upgrade, tick)),
        ),
        CommandData::BuildSquad(build_squad) => (
            build_squad.player_id,
            BuildSquad(build_squad_from_data(build_squad, tick)),
        ),
        CommandData::CancelConstruction(cancel_construction) => (
            cancel_construction.player_id,
            CancelConstruction(cancel_construction_from_data(cancel_construction, tick)),
        ),
        CommandData::CancelProduction(cancel_production) => (
            cancel_production.player_id,
            CancelProduction(cancel_production_from_data(cancel_production, tick)),
        ),
        CommandData::SelectBattlegroup(select_battlegroup) => (
            select_battlegroup.player_id,
            SelectBattlegroup(select_battlegroup_from_data(select_battlegroup, tick)),
        ),
        CommandData::SelectBattlegroupAbility(select_battlegroup_ability) => (
            select_battlegroup_ability.player_id,
            SelectBattlegroupAbility(select_battlegroup_ability_from_data(
                select_battlegroup_ability,
                tick,
            )),
        ),
        CommandData::UseAbility(use_ability) => (
            use_ability.player_id,
            UseAbility(use_ability_from_data(use_ability, tick)),
        ),
        CommandData::UseBattlegroupAbility(use_battlegroup_ability) => (
            use_battlegroup_ability.player_id,
            UseBattlegroupAbility(use_battlegroup_ability_from_data(
                use_battlegroup_ability,
                tick,
            )),
        ),
        CommandData::Unknown(unknown) => {
            (unknown.player_id, Unknown(unknown_from_data(unknown, tick)))
        }
    }
}

pub(crate) fn commands_from_data(data: &[&Tick], player_id: u32) -> Vec<Command> {
    let mut tick_count = 0;

    data.iter()
        .flat_map(|tick| {
            tick_count += 1;

            match tick {
                Tick::Command(command_tick) => command_tick
                    .bundles
                    .iter()
                    .map(|bundle| {
                        let (command_player_id, command) =
                            command_from_data(&bundle.command, tick_count);
                        if player_id == command_player_id as u32 {
                            Some(command)
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![None],
            }
        })
        .filter(|entry| entry.is_some())
        .map(|entry| match entry {
            Some(command) => command,
            None => panic!(),
        })
        .collect()
}

// this is safe as Command does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Command {}
