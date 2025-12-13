//! Utilities for entries in a file tree.
pub use attributes::Attributes;
use std::io;
use std::path::Path;

pub mod attributes;

/// Represents an entry in a file tree and provides utilities for working with it.
pub struct Entry<P: AsRef<Path>> {
    /// The path of this entry.
    path: P,
    /// The file object. Either a directory or a file.
    attributes: Attributes,
}

impl<P> Entry<P>
where
    P: AsRef<Path>,
{
    /// Creates a new [`Entry`]. `path` should be the full path to the entry from the tree's root.
    #[inline]
    pub fn new(path: P) -> io::Result<Self> {
        let attributes = Attributes::new(path.as_ref())?;
        let entry = Self { path, attributes };
        Ok(entry)
    }

    /// Gets the path of this entry.
    #[inline]
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Gets the attributes of this entry.
    #[inline]
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Gets if the entry is executable.
    #[inline]
    pub fn is_executable(&self) -> bool {
        self.attributes.is_executable()
    }

    /// Is the file a dotfile?
    ///
    /// On Unix, this means that the file is hidden.
    pub fn is_dotfile(&self) -> bool {
        let path = self.path.as_ref();
        path.file_name()
            .is_some_and(|filename| filename.as_encoded_bytes().starts_with(b"."))
    }

    /// Is the file hidden?
    ///
    /// On Windows, this checks the file attribute. On Unix, it checks if it is a
    /// dotfile.
    pub fn is_hidden(&self) -> bool {
        self.is_dotfile_hidden() || self.attributes.is_hidden()
    }

    /// Is the file hidden because it's a dotfile? Returns `true` if it is a dotfile
    /// on Unix.
    #[cfg(not(windows))]
    #[inline]
    fn is_dotfile_hidden(&self) -> bool {
        self.is_dotfile()
    }

    /// Is the file hidden because it's a dotfile? Always returns `false` on Windows.
    #[inline]
    #[cfg(windows)]
    fn is_dotfile_hidden(&self) -> bool {
        false
    }
}
