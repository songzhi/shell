use std::path::PathBuf;
use std::string::ToString;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Value {
    Nothing,
    /// A "big int", an integer with arbitrarily large size (aka not limited to 64-bit)
    Int(BigInt),
    /// A "big decimal", an decimal number with arbitrarily large size (aka not limited to 64-bit)
    Number(BigDecimal),
    /// A string value
    String(String),
    /// A glob pattern, eg foo*
    Pattern(String),
    /// A file path
    Path(PathBuf),
    Boolean(bool),
    List(Vec<Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Number(i) => i.to_string(),
            Value::String(s) => s.clone(),
            Value::Pattern(s) => s.clone(),
            Value::Path(s) => s.to_string_lossy().to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::List(v) => v.iter().map(Self::to_string).collect::<Vec<_>>().join(" "),
            Value::Nothing => String::new(),
        }
    }
}
