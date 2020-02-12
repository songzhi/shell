use std::path::PathBuf;

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
