use derive_new::new;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::parser::span::Span;
use crate::parser::token::SpannedToken;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, new)]
pub struct Call {
    pub head: SpannedToken,
    pub positional: Option<Vec<SpannedToken>>,
    pub named: Option<NamedArguments>,
    pub span: Span,
}

impl Call {
    pub fn switch_preset(&self, switch: &str) -> bool {
        self.named
            .as_ref()
            .and_then(|n| n.get(switch))
            .map(|t| match t {
                NamedValue::PresentSwitch(_) => true,
                _ => false,
            })
            .unwrap_or(false)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NamedArguments {
    pub named: IndexMap<String, NamedValue>,
}

impl NamedArguments {
    pub fn new() -> NamedArguments {
        Default::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &NamedValue)> {
        self.named.iter()
    }

    pub fn get(&self, name: &str) -> Option<&NamedValue> {
        self.named.get(name)
    }
}

impl NamedArguments {
    pub fn insert_switch(&mut self, name: impl Into<String>, switch: Option<Span>) {
        let name = name.into();
        match switch {
            None => self.named.insert(name, NamedValue::AbsentSwitch),
            Some(flag) => self.named.insert(name, NamedValue::PresentSwitch(flag)),
        };
    }

    pub fn insert_optional(&mut self, name: impl Into<String>, expr: Option<SpannedToken>) {
        match expr {
            None => self.named.insert(name.into(), NamedValue::AbsentValue),
            Some(expr) => self.named.insert(name.into(), NamedValue::Value(expr)),
        };
    }

    pub fn insert_mandatory(&mut self, name: impl Into<String>, expr: SpannedToken) {
        self.named.insert(name.into(), NamedValue::Value(expr));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NamedValue {
    AbsentSwitch,
    PresentSwitch(Span),
    AbsentValue,
    Value(SpannedToken),
}
