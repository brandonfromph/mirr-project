#![forbid(unsafe_code)]

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum InvocationInputValue {
    String(String),
    Number(f64),
    StringArray(Vec<String>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvocationInputBody {
    values: BTreeMap<String, InvocationInputValue>,
}

impl InvocationInputBody {
    pub fn set_string(&mut self, key: &str, value: impl Into<String>) {
        self.values.insert(key.to_owned(), InvocationInputValue::String(value.into()));
    }

    pub fn set_number(&mut self, key: &str, value: f64) {
        self.values.insert(key.to_owned(), InvocationInputValue::Number(value));
    }

    pub fn set_string_array(&mut self, key: &str, value: Vec<String>) {
        self.values.insert(key.to_owned(), InvocationInputValue::StringArray(value));
    }

    pub fn get(&self, key: &str) -> Option<&InvocationInputValue> {
        self.values.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &InvocationInputValue)> {
        self.values.iter().map(|(key, value)| (key.as_str(), value))
    }
}

pub fn get_body_string(body: &InvocationInputBody, key: &str, fallback: &str) -> String {
    match body.get(key) {
        Some(InvocationInputValue::String(value)) => value.clone(),
        _ => fallback.to_owned(),
    }
}

pub fn get_body_number(body: &InvocationInputBody, key: &str, fallback: f64) -> f64 {
    match body.get(key) {
        Some(InvocationInputValue::Number(value)) if value.is_finite() => *value,
        Some(InvocationInputValue::String(value)) => value.parse::<f64>().unwrap_or(fallback),
        _ => fallback,
    }
}

pub fn get_body_string_array(body: &InvocationInputBody, key: &str) -> Option<Vec<String>> {
    match body.get(key) {
        Some(InvocationInputValue::StringArray(values)) => Some(values.clone()),
        _ => None,
    }
}
