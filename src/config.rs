use crate::event_handler::Result;
use crate::model::*;
use crate::view::Theme;
use platform_dirs::AppDirs;
use ratatui::style::Style;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use toml::Table;
use toml::Value;
pub mod keybind;
use keybind::{get_message, KeybindMap};

pub struct Config {
    pub keybindings: KeybindMap,
    pub theme: Theme,
    pub seek_seconds: i64,
    pub mpd_address: Option<String>,
    pub screens: Vec<Screen>,
    pub nucleo_prefer_prefix: bool,
}

impl Config {
    pub fn default() -> Self {
        Config {
            keybindings: KeybindMap::default(),
            theme: Theme::new(),
            seek_seconds: 5,
            mpd_address: None,
            screens: vec![Screen::Library, Screen::Queue],
            nucleo_prefer_prefix: false,
        }
    }

    pub fn try_read_config(mut self) -> Result<Self> {
        let app_dirs = AppDirs::new(Some("inori"), true);
        let config_file_path =
            app_dirs.map(|d| d.config_dir.join("config.toml"));

        if let Some(Ok(contents)) = config_file_path.map(fs::read_to_string) {
            let toml = contents.parse::<Table>()?; //failed to parse toml
            for (key, value) in toml {
                match (key.as_str(), value) {
                    ("keybindings", Value::Table(t)) => {
                        self.read_keybinds(t)?
                    }
                    ("seek_seconds", Value::Integer(k)) if k > 0 => {
                        self.seek_seconds = k
                    }
                    ("theme", Value::Table(t)) => {
                        self.theme = self.theme.apply_theme(t)?
                    }
                    ("dvorak_keybindings", Value::Boolean(true)) => {
                        self.keybindings = self.keybindings.with_dvorak_style();
                    }
                    ("qwerty_keybindings", Value::Boolean(true)) => {
                        self.keybindings = self.keybindings.with_qwerty_style();
                    }
                    ("mpd_address", Value::String(addr)) => {
                        self.mpd_address = Some(addr);
                    }
                    ("screens", Value::Array(screens)) => {
                        self.screens = screens
                            .iter()
                            .map(|v| match v {
                                Value::String(s) => Ok(Screen::from(s)),
                                x => Err(Box::new(
                                    ConfigError::WrongKeyValueType(
                                        key.to_owned(),
                                        x.to_owned(),
                                    ),
                                )
                                    as Box<dyn Error>),
                            })
                            .collect::<Result<Vec<Screen>>>()?;
                    }
                    ("nucleo_prefer_prefix", Value::Boolean(t)) => {
                        self.nucleo_prefer_prefix = t
                    }
                    (_k, _v) => panic!("unknown key {} or value {}", _k, _v),
                }
            }
        }
        Ok(self)
    }

    pub fn read_keybinds(&mut self, t: Table) -> Result<()> {
        for (key, value) in t {
            match (get_message(&key), value) {
                (Some(m), Value::String(s)) => {
                    let keybinds = keybind::parse_keybind(s).unwrap();
                    self.keybindings.insert(m.clone(), &keybinds);
                }
                (Some(m), Value::Array(a)) => {
                    for v in a {
                        if let Value::String(s) = v {
                            let keybinds = keybind::parse_keybind(s).unwrap();
                            self.keybindings.insert(m.clone(), &keybinds);
                        } else {
                            return Err(Box::new(
                                ConfigError::WrongKeyValueType(key, v),
                            ));
                        }
                    }
                }
                (Some(_m), other) => {
                    return Err(Box::new(ConfigError::WrongKeyValueType(
                        key, other,
                    )))
                }
                (None, _) => {
                    return Err(Box::new(ConfigError::MissingMessage(key)))
                }
            }
        }
        Ok(())
    }
}

//does not log or throw. it simply ignores 'bad' mods
fn join_modifier_array(modifiers: &[Value]) -> String {
    let modifier_strings: Vec<String> = modifiers
        .iter()
        .filter_map(|m| m.as_str().map(|s| s.to_string()))
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
    //UnknownModifier(String),
    UnknownThemeOption(String),
    WrongKeyValueType(String, Value),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingMessage(s) => {
                write!(f, "message {} does not exist", s)
            }
            //ConfigError::UnknownModifier(s) => write!(f, "Error while parsing theme modifier array: unknown modifier: {}", s),
            ConfigError::UnknownThemeOption(s) => {
                write!(f, "theme option {} not found", s)
            }
            ConfigError::WrongKeyValueType(key, s) => {
                write!(f, "keybind {} for command {} has wrong type", s, key)
            }
        }
    }
}

impl Error for ConfigError {}
