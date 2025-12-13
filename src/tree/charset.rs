//! Module for collections of `char`s.

/// Provides text used for generating a tree. Could be considered the "branches" of the
/// tree.
///
/// When implementing this, ideally `depth`, `breadth`, and `indent` should all be the
/// same visual length.
#[non_exhaustive]
pub struct Charset<'a> {
    /// The text to print when traveling deeper into the directory structure.
    ///
    /// Typically should resemble a horizontal line.
    pub depth: &'a str,
    /// The text to print when traversing the breadth of a directory.
    ///
    /// Typically a vertical line. Also helps control padding between branches.
    pub breadth: &'a str,
    /// The text to use to indent tree branches with each level.
    pub indent: &'a str,
}

const EMPTY_TEXT: &str = "    ";

impl<'a> Charset<'a> {
    /// The standard charset. Pretty characters, but not too fancy.
    pub const STANDARD: Self = Self {
        depth: "├── ",
        // NOTE U+00A0 is a non-breaking space
        breadth: "│\u{00A0}\u{00A0} ",
        indent: "    ",
    };

    /// Empty charset. The tree is invisible.
    pub const EMPTY: Self = Self {
        depth: EMPTY_TEXT,
        breadth: EMPTY_TEXT,
        indent: EMPTY_TEXT,
    };
}

impl<'a> Default for Charset<'a> {
    #[inline]
    fn default() -> Self {
        Charset::STANDARD
    }
}
