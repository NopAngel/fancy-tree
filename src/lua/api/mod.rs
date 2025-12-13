//! Module for creating the `fancytree` API for Lua.
use mlua::Lua;

mod path;

/// Creates the table that can be used as the API.
pub fn create(lua: &Lua) -> mlua::Result<mlua::Table> {
    let api = lua.create_table()?;
    api.set("is_unix", IS_UNIX)?;
    api.set("os", OS)?;

    let path_api = path::create(lua)?;
    api.set("path", path_api)?;

    Ok(api)
}

const IS_UNIX: bool = cfg!(unix);

const OS: &str = os_name();

const fn os_name() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "other"
    }
}

#[cfg(test)]
mod tests;
