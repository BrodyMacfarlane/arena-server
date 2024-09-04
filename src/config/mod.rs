use std::sync::OnceLock;
use std::io;
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub player_definitions: PlayerDefinitions,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub tick_rate: u32,
    pub max_connections: u32,
}

#[derive(Deserialize, Debug)]
pub struct PlayerDefinitions {
    pub name: NameConfig,
    pub sprite: SpriteConfig,
    pub stats: StatsConfig,
    pub actions: ActionsConfig,
}

#[derive(Deserialize, Debug)]
pub struct NameConfig {
    pub default: String,
}

#[derive(Deserialize, Debug)]
pub struct SpriteConfig {
    pub default_color: String,
}

#[derive(Deserialize, Debug)]
pub struct StatsConfig {
    pub base_move_speed: f32,
    pub base_fire_rate: f32,
    pub base_damage: u32,
    pub base_health: u32,
    pub base_armor: u32,
}

#[derive(Deserialize, Debug)]
pub struct ActionsConfig {
    pub base_dash_cooldown: f32,
    pub base_backflip_cooldown: f32,
}

fn read_config() -> io::Result<Config> {
    let content = std::fs::read_to_string("src/config/game.toml")?;
    toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        read_config().expect("Failed to load game configuration")
    })
}
