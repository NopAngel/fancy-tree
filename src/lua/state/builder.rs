//! Module for the state builder.
use super::State;
use mlua::Lua;

/// Builds the Lua state.
pub struct Builder;

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self
    }

    /// Builds the Lua state.
    #[must_use]
    pub fn build(self) -> mlua::Result<State> {
        use mlua::{LuaOptions, StdLib};
        let inner = Lua::new_with(StdLib::TABLE | StdLib::STRING, LuaOptions::default())?;
        let state = State { inner };
        let state = state.initialize()?;
        Ok(state)
    }
}
