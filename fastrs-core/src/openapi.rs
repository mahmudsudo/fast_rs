use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, Default, Clone)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub paths: BTreeMap<String, BTreeMap<String, Operation>>,
}

impl OpenApi {
    pub fn new() -> Self {
        Self {
            openapi: "3.0.3".into(),
            info: Info::default(),
            paths: BTreeMap::new(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Info {
    pub title: String,
    pub version: String,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            title: "fastrs API".into(),
            version: "0.1.0".into(),
        }
    }
}

#[derive(Serialize, Default, Clone)]
pub struct Operation {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,
    pub responses: BTreeMap<String, Response>,
}

#[derive(Serialize, Clone)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
    pub required: bool,
    pub schema: Schema,
}

#[derive(Serialize, Clone)]
pub struct RequestBody {
    pub content: BTreeMap<String, MediaType>,
    pub required: bool,
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub description: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub content: BTreeMap<String, MediaType>,
}

#[derive(Serialize, Clone)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Serialize, Default, Clone)]
pub struct Schema {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, Schema>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
}

pub trait OpenApiType {
    fn schema() -> Schema;
}

pub trait OpenApiExtractor {
    fn modify_operation(op: &mut Operation);
}

pub trait OpenApiResponder {
    fn modify_operation(op: &mut Operation);
}

impl OpenApiType for u64 {
    fn schema() -> Schema {
        Schema {
            type_: Some("integer".into()),
            ..Default::default()
        }
    }
}

impl OpenApiType for i64 {
    fn schema() -> Schema {
        Schema {
            type_: Some("integer".into()),
            ..Default::default()
        }
    }
}

impl OpenApiType for i32 {
    fn schema() -> Schema {
        Schema {
            type_: Some("integer".into()),
            ..Default::default()
        }
    }
}

impl OpenApiType for String {
    fn schema() -> Schema {
        Schema {
            type_: Some("string".into()),
            ..Default::default()
        }
    }
}

impl OpenApiType for u32 {
    fn schema() -> Schema {
        Schema {
            type_: Some("integer".into()),
            ..Default::default()
        }
    }
}

impl OpenApiType for bool {
    fn schema() -> Schema {
        Schema {
            type_: Some("boolean".into()),
            ..Default::default()
        }
    }
}

impl<T: OpenApiType> OpenApiType for Option<T> {
    fn schema() -> Schema {
        T::schema()
    }
}

impl<T: OpenApiType> OpenApiType for Vec<T> {
    fn schema() -> Schema {
        Schema {
            type_: Some("array".into()),
            ..Default::default()
        }
    }
}

impl OpenApiType for serde_json::Value {
    fn schema() -> Schema {
        Schema {
            type_: Some("object".into()),
            ..Default::default()
        }
    }
}
