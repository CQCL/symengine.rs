use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use super::expr::Expression;

use symengine_sys::*;

#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

pub trait ExpressionMapKey: Clone + PartialEq + Eq + Hash {
    fn as_str(&self) -> &str;
}

impl ExpressionMapKey for String {
    fn as_str(&self) -> &str {
        self
    }
}

impl<'a> ExpressionMapKey for &'a str {
    fn as_str(&self) -> &str {
        self
    }
}

pub struct ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    basic: *mut CMapBasicBasic,
    table: HashMap<K, Expression>,
}

impl<K> ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K> Default for ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    fn default() -> Self {
        Self {
            basic: unsafe { mapbasicbasic_new() },
            table: HashMap::new(),
        }
    }
}

impl<K> Drop for ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    fn drop(&mut self) {
        unsafe { mapbasicbasic_free(self.basic) }
    }
}

impl<K> fmt::Debug for ExpressionMap<K>
where
    K: ExpressionMapKey + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.table, f)
    }
}

impl<K> ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    pub fn insert<V>(&mut self, key: K, value: V)
    where
        V: Into<Expression>,
    {
        let key_expr = Expression::new(key.as_str());
        unsafe {
            mapbasicbasic_insert(self.basic, key_expr.basic.get(), value.into().basic.get());
        }
        self.table.insert(key, key_expr);
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.table.contains_key(key)
    }

    pub fn eval(&self, expr: &Expression) -> Expression {
        let out = Expression::default();
        unsafe {
            basic_subs(out.basic.get(), expr.basic.get(), self.basic);
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> u64 {
        unsafe { mapbasicbasic_size(self.basic) }
    }
}

impl<K> PartialEq for ExpressionMap<K>
where
    K: ExpressionMapKey + Serialize,
{
    fn eq(&self, other: &Self) -> bool {
        self.table.eq(&other.table)
    }
}

#[cfg(feature = "serde")]
impl<K> Serialize for ExpressionMap<K>
where
    K: ExpressionMapKey + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.table.len()))?;
        for (k, v) in &self.table {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, K> Deserialize<'de> for ExpressionMap<K>
where
    K: ExpressionMapKey + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let table = HashMap::<K, Expression>::deserialize(deserializer)?;

        let mut map = Self::default();
        for (k, v) in &table {
            map.insert(k.clone(), v.clone());
        }
        Ok(map)
    }
}
