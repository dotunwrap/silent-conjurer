use serde::Deserialize;
use std::{env::current_dir, fs::read_to_string, process::exit};

#[derive(Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub roles: RoleConfig,
    pub channels: ChannelConfig,
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub prefix: String,
}

#[derive(Deserialize)]
pub struct RoleConfig {
    pub master_role_id: u64,
    pub commander_role_id: u64,
    pub sergeant_role_id: u64,
    pub officer_role_id: u64,
    pub veteran_role_id: u64,
    pub member_role_id: u64,
    pub recruit_role_id: u64,
    pub guest_role_id: u64,
    pub registrant_role_id: u64,
    pub jobs: JobRoleConfig,
}

#[derive(Deserialize)]
pub struct JobRoleConfig {
    pub divider_role_id: u64,
    pub paladin_role_id: u64,
    pub warrior_role_id: u64,
    pub dark_knight_role_id: u64,
    pub gunbreaker_role_id: u64,
    pub white_mage_role_id: u64,
    pub scholar_role_id: u64,
    pub astrologian_role_id: u64,
    pub sage_role_id: u64,
    pub monk_role_id: u64,
    pub dragoon_role_id: u64,
    pub ninja_role_id: u64,
    pub samurai_role_id: u64,
    pub reaper_role_id: u64,
    pub viper_role_id: u64,
    pub bard_role_id: u64,
    pub machinist_role_id: u64,
    pub dancer_role_id: u64,
    pub black_mage_role_id: u64,
    pub summoner_role_id: u64,
    pub red_mage_role_id: u64,
    pub pictomancer_role_id: u64,
    pub blue_mage_role_id: u64,
}

impl JobRoleConfig {
    pub fn to_vec(&self) -> Vec<u64> {
        // This sucks. I hate it here.
        // Also this doesn't return the divider role
        vec![
            self.paladin_role_id,
            self.warrior_role_id,
            self.dark_knight_role_id,
            self.gunbreaker_role_id,
            self.white_mage_role_id,
            self.scholar_role_id,
            self.astrologian_role_id,
            self.sage_role_id,
            self.monk_role_id,
            self.dragoon_role_id,
            self.ninja_role_id,
            self.samurai_role_id,
            self.reaper_role_id,
            self.viper_role_id,
            self.bard_role_id,
            self.machinist_role_id,
            self.dancer_role_id,
            self.black_mage_role_id,
            self.summoner_role_id,
            self.red_mage_role_id,
            self.pictomancer_role_id,
            self.blue_mage_role_id,
        ]
    }
}

#[derive(Deserialize)]
pub struct ChannelConfig {
    pub officers_channel_id: u64,
    pub general_channel_id: u64,
    pub registrant_channel_id: u64,
    pub registration_notification_channel_id: u64,
}

pub fn load_config() -> Config {
    let config_file = current_dir().unwrap_or_default().join("src/config.toml");

    let contents = match read_to_string(config_file.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Could not read config file {}: {}",
                config_file.display(),
                e
            );
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Could not parse config file {}: {}",
                config_file.display(),
                e
            );
            exit(1);
        }
    };

    config
}
