#![allow(unused)]

use bigdecimal::ToPrimitive;
use de::Visitor;
use serde::de;

use crate::error::ShellError;
use crate::evaluate::CallInfo;
use crate::evaluate::Value;

#[derive(Debug)]
pub struct DeserializerItem<'de> {
    key_struct_field: Option<(String, &'de str)>,
    val: Value,
}

pub struct ConfigDeserializer<'de> {
    call: CallInfo,
    stack: Vec<DeserializerItem<'de>>,
    saw_root: bool,
    position: usize,
}

impl<'de> ConfigDeserializer<'de> {
    pub fn from_call_info(call: CallInfo) -> ConfigDeserializer<'de> {
        ConfigDeserializer {
            call,
            stack: vec![],
            saw_root: false,
            position: 0,
        }
    }

    pub fn push_val(&mut self, val: Value) {
        self.stack.push(DeserializerItem {
            key_struct_field: None,
            val,
        });
    }

    pub fn push(&mut self, name: &'static str) -> Result<(), ShellError> {
        let value: Option<Value> = if name == "rest" {
            let positional = self.call.args.slice_from(self.position);
            self.position += positional.len();
            Some(Value::List(positional))
        } else if self.call.args.has(name) {
            self.call.args.get(name).cloned()
        } else {
            let position = self.position;
            self.position += 1;
            self.call.args.nth(position).cloned()
        };

        self.stack.push(DeserializerItem {
            key_struct_field: Some((name.to_string(), name)),
            val: value.unwrap_or(Value::Nothing),
        });

        Ok(())
    }

    pub fn top(&mut self) -> &DeserializerItem {
        let value = self.stack.last();
        value.expect("Can't get top element of an empty stack")
    }

    pub fn pop(&mut self) -> DeserializerItem {
        let value = self.stack.pop();
        value.expect("Can't pop an empty stack")
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut ConfigDeserializer<'de> {
    type Error = ShellError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.pop();
        match value.val {
            Value::Nothing => visitor.visit_bool(false),
            Value::Boolean(b) => visitor.visit_bool(b),
            _ => Err(ShellError::runtime_error("expected Boolean ")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.pop();
        let val = match value.val {
            Value::Int(i) => i.to_i64(),
            Value::Number(i) => i.to_i64(),
            Value::String(s) => s.parse().ok(),
            Value::Boolean(b) => Some(b as i64),
            Value::List(_) | Value::Nothing | Value::Path(_) | Value::Pattern(_) => None,
        }
        .ok_or(ShellError::runtime_error("expected Integer"))?;
        visitor.visit_i64(val)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.pop();
        let val = match value.val {
            Value::Int(i) => i.to_f64(),
            Value::Number(i) => i.to_f64(),
            Value::String(s) => s.parse().ok(),
            Value::Boolean(b) => Some(b as i8 as f64),
            Value::List(_) | Value::Nothing | Value::Path(_) | Value::Pattern(_) => None,
        }
        .ok_or(ShellError::runtime_error("expected Number"))?;
        visitor.visit_f64(val)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.pop();
        let val = match value.val {
            Value::Nothing => String::new(),
            Value::Int(i) => i.to_string(),
            Value::Number(i) => i.to_string(),
            Value::String(s) => s,
            Value::Pattern(p) => p,
            Value::Path(p) => p.to_string_lossy().to_string(),
            Value::Boolean(b) => b.to_string(),
            _ => return Err(ShellError::runtime_error("expected String")),
        };
        visitor.visit_string(val)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.top();
        match &value.val {
            Value::Nothing => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.pop();
        match value.val {
            Value::List(items) => {
                let de = SeqDeserializer::new(&mut self, items.into_iter());
                visitor.visit_seq(de)
            }
            _ => Err(ShellError::runtime_error("expected Vec")),
        }
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.pop();
        match value.val {
            Value::List(items) => {
                let de = SeqDeserializer::new(&mut self, items.into_iter());
                visitor.visit_seq(de)
            }
            _ => Err(ShellError::runtime_error("expected Tuple")),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(StructDeserializer::new(&mut self, fields))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn is_human_readable(&self) -> bool {
        unimplemented!()
    }
}

struct SeqDeserializer<'a, 'de: 'a, I: Iterator<Item = Value>> {
    de: &'a mut ConfigDeserializer<'de>,
    vals: I,
}

impl<'a, 'de: 'a, I: Iterator<Item = Value>> SeqDeserializer<'a, 'de, I> {
    fn new(de: &'a mut ConfigDeserializer<'de>, vals: I) -> Self {
        SeqDeserializer { de, vals }
    }
}

impl<'a, 'de: 'a, I: Iterator<Item = Value>> de::SeqAccess<'de> for SeqDeserializer<'a, 'de, I> {
    type Error = ShellError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let next = if let Some(next) = self.vals.next() {
            next
        } else {
            return Ok(None);
        };

        self.de.push_val(next);
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.vals.size_hint().1
    }
}

struct StructDeserializer<'a, 'de: 'a> {
    de: &'a mut ConfigDeserializer<'de>,
    fields: &'static [&'static str],
}

impl<'a, 'de: 'a> StructDeserializer<'a, 'de> {
    fn new(de: &'a mut ConfigDeserializer<'de>, fields: &'static [&'static str]) -> Self {
        StructDeserializer { de, fields }
    }
}

impl<'a, 'de: 'a> de::SeqAccess<'de> for StructDeserializer<'a, 'de> {
    type Error = ShellError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.fields.is_empty() {
            return Ok(None);
        }

        self.de.push(self.fields[0])?;
        self.fields = &self.fields[1..];
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.fields.len())
    }
}
