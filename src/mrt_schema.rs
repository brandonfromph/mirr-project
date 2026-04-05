#![forbid(unsafe_code)]

use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaPolicy {
    strict_fail_closed: bool,
    max_fields: usize,
}

impl SchemaPolicy {
    pub fn strict_fail_closed() -> Self {
        Self { strict_fail_closed: true, max_fields: usize::MAX }
    }

    pub fn with_max_fields(mut self, max_fields: usize) -> Self {
        self.max_fields = max_fields;
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SchemaStage {
    ParseJson,
    EnforceFieldBounds,
    EnforceTypes,
}

impl SchemaStage {
    pub fn parse_json() -> Self {
        Self::ParseJson
    }

    pub fn enforce_field_bounds() -> Self {
        Self::EnforceFieldBounds
    }

    pub fn enforce_types() -> Self {
        Self::EnforceTypes
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FieldKind {
    String { max_len: usize },
    U64 { min: u64, max: u64 },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BoundedFieldSpec {
    field: String,
    kind: FieldKind,
}

impl BoundedFieldSpec {
    pub fn string(field: &str, max_len: usize) -> Self {
        Self { field: field.to_string(), kind: FieldKind::String { max_len } }
    }

    pub fn u64(field: &str, min: u64, max: u64) -> Self {
        Self { field: field.to_string(), kind: FieldKind::U64 { min, max } }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaContract {
    route: String,
    required_fields: Vec<BoundedFieldSpec>,
    optional_fields: Vec<BoundedFieldSpec>,
}

impl SchemaContract {
    pub fn new(route: &str) -> Self {
        Self { route: route.to_string(), required_fields: Vec::new(), optional_fields: Vec::new() }
    }

    pub fn with_required_field(mut self, spec: BoundedFieldSpec) -> Self {
        self.required_fields.push(spec);
        self
    }

    pub fn with_optional_field(mut self, spec: BoundedFieldSpec) -> Self {
        self.optional_fields.push(spec);
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FieldValue {
    String(String),
    U64(u64),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BoundedJsonInput {
    fields: HashMap<String, FieldValue>,
    raw_fragment: Option<String>,
}

impl BoundedJsonInput {
    pub fn new() -> Self {
        Self { fields: HashMap::new(), raw_fragment: None }
    }

    pub fn from_raw_fragment(fragment: &str) -> Self {
        Self { fields: HashMap::new(), raw_fragment: Some(fragment.to_string()) }
    }

    pub fn with_string(mut self, field: &str, value: &str) -> Self {
        self.fields.insert(field.to_string(), FieldValue::String(value.to_string()));
        self
    }

    pub fn with_u64(mut self, field: &str, value: u64) -> Self {
        self.fields.insert(field.to_string(), FieldValue::U64(value));
        self
    }

    fn from_json_object(value: Value) -> Option<Self> {
        let mut fields = HashMap::new();
        let object = value.as_object()?;
        for (key, json_value) in object {
            if let Some(s) = json_value.as_str() {
                fields.insert(key.clone(), FieldValue::String(s.to_string()));
                continue;
            }
            if let Some(n) = json_value.as_u64() {
                fields.insert(key.clone(), FieldValue::U64(n));
                continue;
            }
            return None;
        }

        Some(Self { fields, raw_fragment: None })
    }
}

impl Default for BoundedJsonInput {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreDispatchInput {
    route: String,
    payload: BoundedJsonInput,
}

impl PreDispatchInput {
    pub fn new(route: &str, payload: BoundedJsonInput) -> Self {
        Self { route: route.to_string(), payload }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PreDispatchDecision {
    Accept { route: String },
    Reject { code: &'static str, field: Option<String> },
}

#[derive(Debug, Clone)]
pub struct SchemaPipeline {
    policy: SchemaPolicy,
    stages: Vec<SchemaStage>,
    contracts: HashMap<String, SchemaContract>,
}

impl SchemaPipeline {
    pub fn new(policy: SchemaPolicy) -> Self {
        Self { policy, stages: Vec::new(), contracts: HashMap::new() }
    }

    pub fn with_stage(mut self, stage: SchemaStage) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn register_contract(&mut self, contract: SchemaContract) {
        self.contracts.insert(contract.route.clone(), contract);
    }

    pub fn pre_dispatch(&self, input: &PreDispatchInput) -> PreDispatchDecision {
        let payload = match self.decode_payload(input) {
            Some(payload) => payload,
            None => {
                return PreDispatchDecision::Reject { code: "invalid_json_fragment", field: None };
            }
        };

        let Some(contract) = self.contracts.get(&input.route) else {
            if self.policy.strict_fail_closed {
                return PreDispatchDecision::Reject { code: "route_schema_not_found", field: None };
            }

            return PreDispatchDecision::Accept { route: input.route.clone() };
        };

        if payload.fields.len() > self.policy.max_fields {
            return PreDispatchDecision::Reject { code: "too_many_fields", field: None };
        }

        for required in &contract.required_fields {
            if !payload.fields.contains_key(&required.field) {
                return PreDispatchDecision::Reject {
                    code: "missing_required_field",
                    field: Some(required.field.clone()),
                };
            }
        }

        for spec in contract.required_fields.iter().chain(contract.optional_fields.iter()) {
            if let Some(actual) = payload.fields.get(&spec.field) {
                match (&spec.kind, actual) {
                    (FieldKind::String { max_len }, FieldValue::String(value)) => {
                        if value.len() > *max_len {
                            return PreDispatchDecision::Reject {
                                code: "field_too_long",
                                field: Some(spec.field.clone()),
                            };
                        }
                    }
                    (FieldKind::U64 { min, max }, FieldValue::U64(value)) => {
                        if value < min || value > max {
                            return PreDispatchDecision::Reject {
                                code: "field_out_of_range",
                                field: Some(spec.field.clone()),
                            };
                        }
                    }
                    _ => {
                        return PreDispatchDecision::Reject {
                            code: "field_type_mismatch",
                            field: Some(spec.field.clone()),
                        };
                    }
                }
            }
        }

        PreDispatchDecision::Accept { route: input.route.clone() }
    }

    fn decode_payload(&self, input: &PreDispatchInput) -> Option<BoundedJsonInput> {
        if !self.stages.contains(&SchemaStage::ParseJson) {
            return Some(input.payload.clone());
        }

        let Some(fragment) = &input.payload.raw_fragment else {
            return Some(input.payload.clone());
        };

        let parsed = serde_json::from_str::<Value>(fragment).ok()?;
        BoundedJsonInput::from_json_object(parsed)
    }
}
