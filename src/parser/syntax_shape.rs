use std::fmt;

use serde::{Deserialize, Serialize};

/// The syntactic shapes that values must match to be passed into a command. You can think of this as the type-checking that occurs when you call a function.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SyntaxShape {
    /// Any syntactic form is allowed
    Any,
    /// Strings and string-like bare words are allowed
    String,
    /// Only a numeric (integer or decimal) value is allowed
    Number,
    /// Only an integer value is allowed
    Int,
    /// A filepath is allowed
    Path,
    /// A glob pattern is allowed, eg `foo*`
    Pattern,
}

impl fmt::Display for SyntaxShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SyntaxShape::Any => "any",
                SyntaxShape::String => "string",
                SyntaxShape::Number => "number",
                SyntaxShape::Int => "integer",
                SyntaxShape::Path => "path",
                SyntaxShape::Pattern => "pattern",
            }
        )
    }
}
