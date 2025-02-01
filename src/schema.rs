use serde_json::Value;
use std::collections::HashMap;

pub struct ConfigSchema {
    required_fields: Vec<String>,
    field_types: HashMap<String, SchemaType>,
    validators: Vec<Box<dyn Validator>>,
}

#[derive(Debug, Clone)]
pub enum SchemaType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

pub trait Validator: Send + Sync {
    fn validate(&self, value: &Value) -> bool;
    fn error_message(&self) -> String;
}
