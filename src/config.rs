use crate::model::*;
use crate::view::Theme;
use crate::event_handler::Result;
use platform_dirs::AppDirs;
use ratatui::style::Modifier;
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
                    ("keybindings", Value::Table(t)) => self.read_keybinds(t)?,
                    ("seek_seconds", Value::Integer(k)) if k > 0 => {
                        self.seek_seconds = k
                    }
                    ("theme", Value::Table(t)) => self.read_theme(t)?,
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
                        let temp_screens: Result<Vec<Screen>> = screens
                            .iter()
                            .map(
                                |v| match v {
                                Value::String(s) => Ok(Screen::from(s)),
                                x => Err(Box::new(ConfigError::WrongKeyValueType(key.to_owned(), x.to_owned())) as Box<dyn Error>),
                                })
                            .collect();
                        self.screens = match temp_screens {
                            Ok(s) => s,
                            Err(e) => return Err(e),
                        };
                    }
                    ("nucleo_prefer_prefix", Value::Boolean(t)) => self.nucleo_prefer_prefix = t,
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
                            return Err(Box::new(ConfigError::WrongKeyValueType(key, v)))
                        }
                    }
                }
                (Some(_m), other) => return Err(Box::new(ConfigError::WrongKeyValueType(key, other))),
                (None, _) => return Err(Box::new(ConfigError::MissingMessage(key))),
            }
        }
        Ok(())
    }

    pub fn read_theme(&mut self, t: Table) -> Result<()> {
        for (key, value) in t {
            match (key.as_str(), value) {
                ("item_highlight_active", Value::Table(t)) => {
                    self.theme.item_highlight_active = deserialize_style(t)?;
                }
                ("item_highlight_inactive", Value::Table(t)) => {
                    self.theme.item_highlight_inactive = deserialize_style(t)?;
                }
                ("block_active", Value::Table(t)) => {
                    self.theme.block_active = deserialize_style(t)?;
                }
                ("status_artist", Value::Table(t)) => {
                    self.theme.status_artist = deserialize_style(t)?;
                }
                ("status_album", Value::Table(t)) => {
                    self.theme.status_album = deserialize_style(t)?;
                }
                ("status_title", Value::Table(t)) => {
                    self.theme.status_title = deserialize_style(t)?;
                }
                ("artist_sort", Value::Table(t)) => {
                    self.theme.field_artistsort = deserialize_style(t)?;
                }
                ("field_artistsort", Value::Table(t)) => {
                    self.theme.field_artistsort = deserialize_style(t)?;
                }
                ("album", Value::Table(t)) => {
                    self.theme.field_album = deserialize_style(t)?;
                }
                ("field_album", Value::Table(t)) => {
                    self.theme.field_album = deserialize_style(t)?;
                }
                ("playing", Value::Table(t)) => {
                    self.theme.status_playing = deserialize_style(t)?;
                }
                ("paused", Value::Table(t)) => {
                    self.theme.status_paused = deserialize_style(t)?;
                }
                ("stopped", Value::Table(t)) => {
                    self.theme.status_stopped = deserialize_style(t)?;
                }
                ("status_playing", Value::Table(t)) => {
                    self.theme.status_playing = deserialize_style(t)?;
                }
                ("status_paused", Value::Table(t)) => {
                    self.theme.status_paused = deserialize_style(t)?;
                }
                ("status_stopped", Value::Table(t)) => {
                    self.theme.status_stopped = deserialize_style(t)?;
                }
                ("slash_span", Value::Table(t)) => {
                    self.theme.slash_span = deserialize_style(t)?;
                }
                ("search_query_active", Value::Table(t)) => {
                    self.theme.search_query_active = deserialize_style(t)?;
                }
                ("search_query_inactive", Value::Table(t)) => {
                    self.theme.search_query_inactive = deserialize_style(t)?;
                }
                ("progress_bar_filled", Value::Table(t)) => {
                    self.theme.progress_bar_filled = deserialize_style(t)?;
                }
                ("progress_bar_unfilled", Value::Table(t)) => {
                    self.theme.progress_bar_unfilled = deserialize_style(t)?;
                }
                (other, _) => return Err(Box::new(ConfigError::UnknownThemeOption(other.to_string()))),
            }
        }
        Ok(())
    }
}

fn modifiers_arr(modifiers: &Vec<Value>) -> String {
    let mut m = String::new();
    for i in modifiers {
        if let Value::String(s) = i {
            if Modifier::from_name(s).is_some() {
                if !m.is_empty() {
                    m.push('|');
                }
                m.push_str(s);
            } else {
                continue //TODO: log this: Error while parsing theme modifier array: unknown modifier
            }
        }
    }
    m
}

//cleaner code, but without trailing pipe. unsure if it's needed.
//this one does not log or throw. it simply ignores 'bad' mods
fn _join_modifier_array(modifiers: &Vec<Value>) -> String {
    let modifier_strings: Vec<String> = modifiers.iter()
        .filter_map(|m| m.as_str())
        .map(|s| s.to_string())
        .collect();

    let mod_string: String = modifier_strings.join("|");
    mod_string
}

pub fn deserialize_style(mut t: Table) -> Result<Style> {
    if !t.contains_key("add_modifier") {
        t.insert("add_modifier".into(), Value::String("".into()));
    }
    if !t.contains_key("sub_modifier") {
        t.insert("sub_modifier".into(), Value::String("".into()));
    }
    if let Value::Array(a) = t.get("add_modifier").unwrap() {
        t.insert("add_modifier".into(), Value::String(modifiers_arr(a)));
    }
    if let Value::Array(a) = t.get("sub_modifier").unwrap() {
        t.insert("sub_modifier".into(), Value::String(modifiers_arr(a)));
    }
    Ok(t.try_into()?)
}

#[derive(Debug)]
enum ConfigError {
    MissingMessage(String),
    //UnknownModifier(String),
    UnknownThemeOption(String),
    WrongKeyValueType(String, Value),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingMessage(s) => write!(f, "message {} does not exist", s),
            //ConfigError::UnknownModifier(s) => write!(f, "Error while parsing theme modifier array: unknown modifier: {}", s),
            ConfigError::UnknownThemeOption(s) => write!(f, "theme option {} not found", s),
            ConfigError::WrongKeyValueType(key, s) => write!(f, "keybind {} for command {} has wrong type", s, key),
        }
    }
}

impl Error for ConfigError {}