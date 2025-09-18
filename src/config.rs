use crate::model::*;
use crate::view::Theme;
use crate::event_handler::Result;
use platform_dirs::AppDirs;
use ratatui::style::Style;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use toml::Table;
use toml::Value;
pub mod keybind;
use keybind::KeybindMap;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keybindings: KeybindMap,
    #[serde(default = "Theme::new")]
    pub theme: Theme,
    #[serde(default = "default_seek")]
    pub seek_seconds: i64,
    pub mpd_address: Option<String>,
    #[serde(default = "default_screens")]
    pub screens: Vec<Screen>,
    #[serde(default = "default_nucleo_prefer_prefix")]
    pub nucleo_prefer_prefix: bool,
}

fn default_seek() -> i64 { 5 }
fn default_screens() -> Vec<Screen> { vec![Screen::Library, Screen::Queue] }
fn default_nucleo_prefer_prefix() -> bool { false }

impl Default for Config {
    fn default() -> Self {
        Config {
            keybindings: KeybindMap::default(),
            theme: Theme::new(),
            seek_seconds: 5,
            mpd_address: None,
            screens: vec![Screen::Library, Screen::Queue],
            nucleo_prefer_prefix: false,
        }
    }
}

impl Config {
    pub fn from_toml() -> Option<Self> {
        let app_dirs = AppDirs::new(Some("inori"), true)?;
        let config_file_path = app_dirs.config_dir.join("config.toml");

        let content = fs::read_to_string(config_file_path).ok()?;
        let config: Config = toml::from_str(&content).expect("wrong fields");
        Some(config)
    }
}

//does not log or throw. it simply ignores 'bad' mods
fn join_modifier_array(modifiers: &Vec<Value>) -> String {
    let modifier_strings: Vec<String> = modifiers.iter()
        .filter_map(|m| m
            .as_str()
            .map(|s| s.to_string()))
        .collect();

    let mod_string: String = modifier_strings.join("|");
    mod_string + "|"
}

pub fn deserialize_style(mut t: Table) -> Result<Style> {
    if !t.contains_key("add_modifier") {
        t.insert("add_modifier".into(), Value::String("".into()));
    }
    if !t.contains_key("sub_modifier") {
        t.insert("sub_modifier".into(), Value::String("".into()));
    }
    if let Value::Array(a) = t.get("add_modifier").unwrap() {
        t.insert("add_modifier".into(), Value::String(join_modifier_array(a)));
    }
    if let Value::Array(a) = t.get("sub_modifier").unwrap() {
        t.insert("sub_modifier".into(), Value::String(join_modifier_array(a)));
    }
    Ok(t.try_into()?)
}

#[derive(Debug)]
pub enum ConfigError {
    MissingMessage(String),
    UnknownModifier(String),
    UnknownThemeOption(String),
    WrongKeyValueType(String, Value),
    MissingFile(String),
    TomlParse(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingMessage(s) => write!(f, "message {} does not exist", s),
            ConfigError::UnknownModifier(s) => write!(f, "Error while parsing theme modifier array: unknown modifier: {}", s),
            ConfigError::UnknownThemeOption(s) => write!(f, "theme option {} not found", s),
            ConfigError::WrongKeyValueType(key, s) => write!(f, "keybind {} for command {} has wrong type", s, key),
            ConfigError::MissingFile(s) => write!(f, "failed to read {}", s),
            ConfigError::TomlParse(s) => write!(f, "failed to parse {}", s),
        }
    }
}

impl Error for ConfigError {}