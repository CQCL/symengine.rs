use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

use super::expr::Expression;

use symengine_sys::*;

#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

pub trait ExpressionMapKey: Clone + PartialEq + Eq + Hash {
    fn to_string(&self) -> String;
}

impl ExpressionMapKey for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl<'a> ExpressionMapKey for &'a str {
    fn to_string(&self) -> String {
        String::from(*self)
    }
}

pub struct ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    basic: *mut CMapBasicBasic,
    table: HashSet<K>,
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
            table: HashSet::new(),
        }
    }
}

impl<K> Clone for ExpressionMap<K>
where
    K: ExpressionMapKey,
{
    fn clone(&self) -> Self {
        let mut new = Self::new();
        for key in &self.table {
            let value = self.get(key).unwrap();
            new.insert(key.clone(), value);
        }
        new
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
        let key_expr = Expression::new(key.to_string());
        unsafe {
            mapbasicbasic_insert(self.basic, key_expr.basic.get(), value.into().basic.get());
        }
        self.table.insert(key);
    }

    pub fn get(&self, key: &K) -> Option<Expression> {
        if self.table.contains(key) {
            let key_expr = Expression::new(key.to_string());
            let value = Expression::default();
            unsafe {
                mapbasicbasic_get(self.basic, key_expr.basic.get(), value.basic.get());
            }
            Some(value)
        } else {
            None
        }
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.table.contains(key)
    }

    pub fn eval_once(&self, expr: &Expression) -> Expression {
        let out = Expression::default();
        unsafe {
            basic_subs(out.basic.get(), expr.basic.get(), self.basic);
        }
        out
    }

    pub fn eval_key(&self, key: &K) -> Option<Expression> {
        self.get(key).map(|v| self.eval_once(&v))
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
        for key in &self.table {
            let value = self.get(key);
            map.serialize_entry(key, &value)?;
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
