use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::syntax_shape::SyntaxShape;

/// The types of named parameter that a command can have
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NamedType {
    /// A flag without any associated argument. eg) `foo --bar`
    Switch,
    /// A mandatory flag, with associated argument. eg) `foo --required xyz`
    Mandatory(SyntaxShape),
    /// An optional flag, with associated argument. eg) `foo --optional abc`
    Optional(SyntaxShape),
}


/// The type of positional arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionalType {
    /// A mandatory positional argument with the expected shape of the value
    Mandatory(String, SyntaxShape),
    /// An optional positional argument with the expected shape of the value
    Optional(String, SyntaxShape),
}

impl PositionalType {
    /// Helper to create a mandatory positional argument type
    pub fn mandatory(name: &str, ty: SyntaxShape) -> PositionalType {
        PositionalType::Mandatory(name.to_string(), ty)
    }

    /// Helper to create a mandatory positional argument with an "any" type
    pub fn mandatory_any(name: &str) -> PositionalType {
        PositionalType::Mandatory(name.to_string(), SyntaxShape::Any)
    }


    /// Helper to create a optional positional argument type
    pub fn optional(name: &str, ty: SyntaxShape) -> PositionalType {
        PositionalType::Optional(name.to_string(), ty)
    }

    /// Helper to create a optional positional argument with an "any" type
    pub fn optional_any(name: &str) -> PositionalType {
        PositionalType::Optional(name.to_string(), SyntaxShape::Any)
    }

    /// Gets the name of the positional argument
    pub fn name(&self) -> &str {
        match self {
            PositionalType::Mandatory(s, _) => s,
            PositionalType::Optional(s, _) => s,
        }
    }

    /// Gets the expected type of a positional argument
    pub fn syntax_type(&self) -> SyntaxShape {
        match *self {
            PositionalType::Mandatory(_, t) => t,
            PositionalType::Optional(_, t) => t,
        }
    }
}

type Description = String;

/// The full signature of a command. All commands have a signature similar to a function signature.
/// Commands will use this information to register themselves with Nu's core engine so that the command
/// can be invoked, help can be displayed, and calls to the command can be error-checked.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    /// The name of the command. Used when calling the command
    pub name: String,
    /// Usage instructions about the command
    pub usage: String,
    /// The list of positional arguments, both required and optional, and their corresponding types and help text
    pub positional: Vec<(PositionalType, Description)>,
    /// After the positional arguments, a catch-all for the rest of the arguments that might follow, their type, and help text
    pub rest_positional: Option<(SyntaxShape, Description)>,
    /// The named flags with corresponding type and help text
    pub named: IndexMap<String, (NamedType, Description)>,
}

impl Signature {
    pub fn shift_positional(&mut self) {
        self.positional = Vec::from(&self.positional[1..]);
    }

    pub fn remove_named(&mut self, name: &str) {
        self.named.remove(name);
    }
}


impl Signature {
    /// Create a new command signature with the given name
    pub fn new(name: impl Into<String>) -> Signature {
        Signature {
            name: name.into(),
            usage: String::new(),
            positional: vec![],
            rest_positional: None,
            named: indexmap::indexmap! {"help".into() => (NamedType::Switch, "Display this help message".into())},
        }
    }

    /// Create a new signature
    pub fn build(name: impl Into<String>) -> Signature {
        Signature::new(name.into())
    }

    /// Add a description to the signature
    pub fn desc(mut self, usage: impl Into<String>) -> Signature {
        self.usage = usage.into();
        self
    }

    /// Add a required positional argument to the signature
    pub fn required(
        mut self,
        name: impl Into<String>,
        ty: impl Into<SyntaxShape>,
        desc: impl Into<String>,
    ) -> Signature {
        self.positional.push((
            PositionalType::Mandatory(name.into(), ty.into()),
            desc.into(),
        ));

        self
    }

    /// Add an optional positional argument to the signature
    pub fn optional(
        mut self,
        name: impl Into<String>,
        ty: impl Into<SyntaxShape>,
        desc: impl Into<String>,
    ) -> Signature {
        self.positional.push((
            PositionalType::Optional(name.into(), ty.into()),
            desc.into(),
        ));

        self
    }

    /// Add an optional named flag argument to the signature
    pub fn named(
        mut self,
        name: impl Into<String>,
        ty: impl Into<SyntaxShape>,
        desc: impl Into<String>,
    ) -> Signature {
        self.named
            .insert(name.into(), (NamedType::Optional(ty.into()), desc.into()));

        self
    }

    /// Add a required named flag argument to the signature
    pub fn required_named(
        mut self,
        name: impl Into<String>,
        ty: impl Into<SyntaxShape>,
        desc: impl Into<String>,
    ) -> Signature {
        self.named
            .insert(name.into(), (NamedType::Mandatory(ty.into()), desc.into()));

        self
    }

    /// Add a switch to the signature
    pub fn switch(mut self, name: impl Into<String>, desc: impl Into<String>) -> Signature {
        self.named
            .insert(name.into(), (NamedType::Switch, desc.into()));
        self
    }

    /// Set the type for the "rest" of the positional arguments
    pub fn rest(mut self, ty: SyntaxShape, desc: impl Into<String>) -> Signature {
        self.rest_positional = Some((ty, desc.into()));
        self
    }
}
