use bevy::prelude::*;
use bimap::BiMap;
use serde::Deserialize;
use std::{fs, str::FromStr};
use toml::{de, from_str};

const CONFIG_PATH: &str = "./resources/config.toml";

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct KeybindConfig {
    forward: String,
    backward: String,
    left: String,
    right: String,
    jump: String,
}

impl Default for KeybindConfig {
    fn default() -> Self {
        KeybindConfig {
            forward: "w".to_owned(),
            backward: "s".to_owned(),
            left: "a".to_owned(),
            right: "d".to_owned(),
            jump: " ".to_owned(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct NetworkConfig {
    host: bool,
    host_address: Option<String>,
    port: u32
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            host: true,
            port: 8080,
            host_address: None
        }
    }
}

#[derive(Deserialize, Resource, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Config {
    network: NetworkConfig,
    keybinds: KeybindConfig,
}

#[derive(thiserror::Error, Debug)]
#[error("Unrecognized Keybind found: {0}")]
pub struct UnrecognizedKeybindError(String);

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    DeserializationError(#[from] de::Error),

    #[error(transparent)]
    FsError(#[from] std::io::Error),

    #[error(transparent)]
    UnrecognizedKeybind(#[from] UnrecognizedKeybindError),
}

#[derive(Hash, PartialEq, Eq)]
pub enum Keybinds {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_config);
    }
}

fn read_config() -> Result<Config, ConfigError> {
    let cfg_chars = &fs::read(CONFIG_PATH)?;
    let cfg_str: String = String::from_utf8_lossy(cfg_chars).to_string();

    Ok(from_str(&cfg_str)?)
}

fn init_config(mut cmd: Commands) {
    let cfg_r = read_config();

    if let Ok(cfg) = cfg_r {
        info!("{:?}", cfg);
        cmd.insert_resource(cfg);
    } else {
        // do some error logging maybe?
    }
}

trait KeycodeFromString {
    fn from_str(str: &str) -> Result<KeyCode, UnrecognizedKeybindError>;
}

impl KeycodeFromString for KeyCode {
    fn from_str(str: &str) -> Result<KeyCode, UnrecognizedKeybindError> {
        match str {
            "w" => Ok(KeyCode::W),
            "a" => Ok(KeyCode::A),
            "s" => Ok(KeyCode::S),
            "d" => Ok(KeyCode::D),
            " " => Ok(KeyCode::Space),
            _ => Err(UnrecognizedKeybindError(str.to_owned())),
        }
    }
}

pub fn build_keybind_map(
    cfg: Config,
) -> Result<BiMap<Keybinds, KeyCode>, UnrecognizedKeybindError> {
    let mut map = BiMap::new();
    map.insert(Keybinds::Forward, KeyCode::from_str(&cfg.keybinds.forward)?);
    map.insert(
        Keybinds::Backward,
        KeyCode::from_str(&cfg.keybinds.backward)?,
    );
    map.insert(Keybinds::Left, KeyCode::from_str(&cfg.keybinds.left)?);
    map.insert(Keybinds::Right, KeyCode::from_str(&cfg.keybinds.right)?);
    map.insert(Keybinds::Jump, KeyCode::from_str(&cfg.keybinds.jump)?);

    return Ok(map);
}

// ████████╗███████╗███████╗████████╗
// ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
//    ██║   █████╗  ███████╗   ██║
//    ██║   ██╔══╝  ╚════██║   ██║
//    ██║   ███████╗███████║   ██║
//    ╚═╝   ╚══════╝╚══════╝   ╚═╝
#[test]
fn parses_empty_file() {
    let empty_file = "";
    let cfg = from_str(empty_file);

    assert_eq!(cfg, Ok(Config::default()))
}

#[test]
fn parses_partial_file() {
    let toml = r#"
        [keybinds]
        forward = 'k'

        [network]
        host = false
        host_address = 'http://game.mushie.io'           
    "#;
    let cfg = from_str(toml);

    assert_eq!(cfg, Ok(Config {
        keybinds: KeybindConfig {
            forward: "k".to_owned(),
            ..KeybindConfig::default()
        },
        network: NetworkConfig {
            host: false,
            host_address: Some("http://game.mushie.io".to_owned()),
            ..NetworkConfig::default()
        }
    }))
}

#[test]
fn parses_full_file() {
    let toml = r#"
        [keybinds]
        forward = 'k'
        backward = 'j'
        left = 'h'
        right = 'l'
        jump = 'o'

        [network]
        port = 1234
        host = false
        host_address = 'http://game.mushie.io'           
    "#;
    let cfg = from_str(toml);

    assert_eq!(cfg, Ok(Config {
        keybinds: KeybindConfig {
            forward: "k".to_owned(),
            backward: "j".to_owned(),
            left: "h".to_owned(),
            right: "l".to_owned(),
            jump: "o".to_owned(),
        },
        network: NetworkConfig {
            host: false,
            host_address: Some("http://game.mushie.io".to_owned()),
            port: 1234
        }
    }))
}
