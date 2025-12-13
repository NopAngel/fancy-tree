//! Module for the icon config.
use super::ConfigFile;
use crate::lua::interop;
use crate::tree::Entry;
use mlua::{FromLua, Lua};
use std::path::Path;

/// The configuration for icons.
#[derive(Debug)]
pub struct Icons {
    /// Function to get the icon for an entry.
    get_icon: mlua::Function,
}

impl Icons {
    /// Get the icon for the entry.
    pub fn get_icon<P>(
        &self,
        entry: &Entry<P>,
        default_choice: &str,
    ) -> mlua::Result<Option<String>>
    where
        P: AsRef<Path>,
    {
        let path = entry.path();
        let attributes = interop::FileAttributes::from(entry);

        self.get_icon.call((path, attributes, default_choice))
    }
}

impl ConfigFile for Icons {
    const FILENAME: &'static str = "icons.lua";
    const DEFAULT_MODULE: &'static str = include_str!("./icons.lua");
}

impl FromLua for Icons {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        mlua::Function::from_lua(value, lua).map(|get_icon| Self { get_icon })
    }
}
