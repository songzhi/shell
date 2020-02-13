use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use derive_new::new;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::evaluate::value::Value;
use crate::shell::Shell;

/// Associated information for the call of a command, including the args passed to the command and a tag that spans the name of the command being called
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct CallInfo {
    /// The arguments associated with this call
    pub args: EvaluatedArgs,
    //    pub name: Span,
}

/// The set of positional and named arguments, after their values have been evaluated.
///
/// * Positional arguments are those who are given as values, without any associated flag. For example, in `foo arg1 arg2`, both `arg1` and `arg2` are positional arguments.
/// * Named arguments are those associated with a flag. For example, `foo --given bar` the named argument would be name `given` and the value `bar`.
#[derive(Debug, Default, Eq, PartialEq, new, Serialize, Deserialize, Clone)]
pub struct EvaluatedArgs {
    pub positional: Option<Vec<Value>>,
    pub named: Option<IndexMap<String, Value>>,
}

impl EvaluatedArgs {
    /// Retrieve a subset of positional arguments starting at a given position
    pub fn slice_from(&self, from: usize) -> Vec<Value> {
        let positional = &self.positional;

        match positional {
            None => vec![],
            Some(list) => list[from..].to_vec(),
        }
    }

    /// Get the nth positional argument, if possible
    pub fn nth(&self, pos: usize) -> Option<&Value> {
        match &self.positional {
            Some(positional) => positional.get(pos),
            None => None,
        }
    }

    /// Get the number of positional arguments available
    pub fn len(&self) -> usize {
        match &self.positional {
            Some(positional) => positional.len(),
            None => 0,
        }
    }

    /// Return if there are no positional arguments
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return true if the set of named arguments contains the name provided
    pub fn has(&self, name: &str) -> bool {
        match &self.named {
            None => false,
            Some(named) => named.contains_key(name),
        }
    }

    /// Gets the corresponding Value for the named argument given, if possible
    pub fn get(&self, name: &str) -> Option<&Value> {
        match &self.named {
            None => None,
            Some(named) => named.get(name),
        }
    }

    /// Iterates over the positional arguments
    pub fn positional_iter(&self) -> PositionalIter<'_> {
        match &self.positional {
            None => PositionalIter::Empty,
            Some(v) => {
                let iter = v.iter();
                PositionalIter::Array(iter)
            }
        }
    }
}

/// An iterator to help iterate over positional arguments
pub enum PositionalIter<'a> {
    Empty,
    Array(std::slice::Iter<'a, Value>),
}

impl<'a> Iterator for PositionalIter<'a> {
    type Item = &'a Value;

    /// The required `next` function to implement the Iterator trait
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PositionalIter::Empty => None,
            PositionalIter::Array(iter) => iter.next(),
        }
    }
}
