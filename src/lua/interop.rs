//! Crate for interoperability between non-config types and the Lua config files.
use crate::tree::Entry;
use crate::tree::entry::Attributes;
use mlua::{IntoLua, Lua};
use std::path::Path;

/// FileAttributes tracks various file stats.
pub struct FileAttributes<'a, P: AsRef<Path>>(&'a Entry<P>);

impl<'a, P> FileAttributes<'a, P>
where
    P: AsRef<Path>,
{
    /// Is the file hidden?
    #[inline]
    fn is_hidden(&self) -> bool {
        self.0.is_hidden()
    }

    /// Is the file an executable?
    #[inline]
    fn is_executable(&self) -> bool {
        self.0.is_executable()
    }

    /// What is the file type (string enum)?
    fn file_type(&self) -> &str {
        const DIRECTORY: &str = "directory";
        const FILE: &str = "file";
        const SYMLINK: &str = "symlink";

        match self.0.attributes() {
            Attributes::Directory(_) => DIRECTORY,
            Attributes::File(_) => FILE,
            Attributes::Symlink(_) => SYMLINK,
        }
    }

    /// The file's code language.
    fn language(&self) -> Option<&'static str> {
        self.0
            .attributes()
            .file()
            .and_then(|file| file.language())
            .map(|language| language.name())
    }
}

impl<'a, P> IntoLua for FileAttributes<'a, P>
where
    P: AsRef<Path>,
{
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("is_hidden", self.is_hidden())?;
        table.set("is_executable", self.is_executable())?;
        table.set("file_type", self.file_type())?;
        table.set("language", self.language())?;
        let table = mlua::Value::Table(table);
        Ok(table)
    }
}

impl<'a, P> From<&'a Entry<P>> for FileAttributes<'a, P>
where
    P: AsRef<Path>,
{
    #[inline]
    fn from(value: &'a Entry<P>) -> Self {
        Self(value)
    }
}
