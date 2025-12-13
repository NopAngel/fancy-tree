//! Module for lua utilities.
use mlua::Lua;

mod api;
pub mod interop;

/// Creates a new Lua state for the application.
pub fn new_state() -> mlua::Result<Lua> {
    use mlua::{LuaOptions, StdLib};

    /// The global name of the API.
    const API_NAME: &str = "fancytree";

    let lua = Lua::new_with(StdLib::TABLE | StdLib::STRING, LuaOptions::default())?;
    let api = api::create(&lua)?;
    let globals = lua.globals();
    globals.set(API_NAME, api)?;
    Ok(lua)
}
