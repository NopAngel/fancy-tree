//! Module for configuring colors.
use super::ConfigFile;
use crate::color::Color;
use crate::lua::interop;
use crate::tree::Entry;
use mlua::{FromLua, Lua};
use std::path::Path;

/// The configuration for application colors.
#[derive(Debug)]
pub struct Colors {
    /// Function to get the color for an entry's icon.
    for_icon: mlua::Function,
}

impl Colors {
    /// Get the color for an entry's icon.
    pub fn for_icon<P>(
        &self,
        entry: &Entry<P>,
        default_choice: Option<Color>,
    ) -> mlua::Result<Option<Color>>
    where
        P: AsRef<Path>,
    {
        let path = entry.path();
        let attributes = interop::FileAttributes::from(entry);

        self.for_icon.call((path, attributes, default_choice))
    }
}

impl ConfigFile for Colors {
    const FILENAME: &'static str = "colors.lua";
    const DEFAULT_MODULE: &'static str = include_str!("./colors.lua");
}

impl FromLua for Colors {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        const FOR_ICON_KEY: &str = "icons";

        let table = mlua::Table::from_lua(value, lua)?;
        let for_icon = table.get::<mlua::Function>(FOR_ICON_KEY)?;

        let colors = Self { for_icon };
        Ok(colors)
    }
}
