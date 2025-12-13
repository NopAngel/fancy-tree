//! Module for creating a Lua state object for the application.
use super::api;
pub use builder::Builder;
use mlua::Lua;

mod builder;

/// Container for the Lua state.
///
/// This helps ensure proper lifetimes for any type that the state uses.
pub struct State {
    /// The actual Lua state.
    inner: Lua,
}

impl State {
    /// Initializes the API for the Lua state.
    #[must_use]
    fn initialize(self) -> mlua::Result<Self> {
        /// The global name of the API.
        const API_NAME: &str = "fancytree";

        let api = api::create(&self.inner)?;
        let globals = self.inner.globals();
        globals.set(API_NAME, api)?;

        Ok(self)
    }

    /// The inner Lua state.
    pub fn to_inner(&self) -> &Lua {
        &self.inner
    }
}
