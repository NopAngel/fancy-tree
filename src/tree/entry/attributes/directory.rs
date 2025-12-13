//! Module for directory attributes.

use super::interop::has_hidden_attribute;
use std::fs::Metadata;

/// Attributes for a directory.
pub struct DirectoryAttributes {
    /// Is the directory hidden?
    hidden: bool,
}

impl DirectoryAttributes {
    /// Creates new directory attributes.
    #[inline]
    pub(super) fn new(metadata: Metadata) -> Self {
        Self {
            hidden: has_hidden_attribute(&metadata),
        }
    }

    /// Is the directory hidden?
    #[inline]
    pub const fn is_hidden(&self) -> bool {
        self.hidden
    }
}
