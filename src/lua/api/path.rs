//! Module for path utilities in Lua.
use mlua::{IntoLua, Lua};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, RwLock};

/// Creates the table for the API utilities under the path namespace.
pub fn create(lua: &Lua) -> mlua::Result<mlua::Table> {
    let api = lua.create_table()?;
    let filename = lua.create_function(|lua, (path,): (String,)| {
        Path::new(&path)
            .file_name()
            .map(|os_str| os_str.into_lua(lua))
            .transpose()
    })?;
    api.set("filename", filename)?;
    let glob_matches = lua.create_function(|_lua, (glob, path): (String, String)| {
        Ok(glob_matches_impl(glob, path))
    })?;
    api.set("glob_matches", glob_matches)?;

    Ok(api)
}

#[inline]
fn glob_matches_impl<S, P>(raw: S, path: P) -> bool
where
    String: From<S>,
    P: AsRef<Path>,
{
    /// Caches compiled globs so that they aren't recompiled.
    static GLOB_MEMO: LazyLock<RwLock<HashMap<String, Option<glob::Pattern>>>> =
        LazyLock::new(|| {
            let map = HashMap::new();
            RwLock::new(map)
        });

    /// Creates a new glob pattern, or `None` if it can't be compiled.
    #[inline]
    fn create_glob(pattern: &str) -> Option<glob::Pattern> {
        glob::Pattern::new(pattern).ok()
    }

    let matches = |glob: &glob::Pattern| glob.matches_path(path.as_ref());

    let raw = String::from(raw);

    // NOTE Ensure that the lock is dropped after it is used.
    {
        let Ok(memo) = GLOB_MEMO.read() else {
            // NOTE We can't pull the cached glob, so we'll just compile on demand and
            //      return.
            return create_glob(&raw).as_ref().is_some_and(matches);
        };
        if let Some(glob) = memo.get(&raw) {
            return glob.as_ref().is_some_and(matches);
        }
    }

    // NOTE We couldn't find the glob, so we'll compile a new one and try to write it
    //      to the cache.
    let glob = create_glob(&raw);
    let result = glob.as_ref().is_some_and(matches);
    if let Ok(mut memo) = GLOB_MEMO.write() {
        memo.insert(raw, glob);
    }
    result
}
