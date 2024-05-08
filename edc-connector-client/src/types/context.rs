use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::EDC_NAMESPACE;

const ODRL_CONTEXT: &str = "http://www.w3.org/ns/odrl.jsonld";

#[derive(Deserialize, Debug)]
pub struct WithContext<T> {
    #[allow(dead_code)]
    #[serde(rename = "@context")]
    context: Value,
    #[serde(flatten)]
    pub(crate) inner: T,
}

#[derive(Serialize, Debug)]
pub struct WithContextRef<'a, T> {
    #[serde(rename = "@context")]
    context: Value,
    #[serde(flatten)]
    inner: &'a T,
}

impl<'a, T> WithContextRef<'a, T> {
    pub fn new(context: Value, inner: &'a T) -> WithContextRef<T> {
        WithContextRef { context, inner }
    }

    pub fn default_context(inner: &'a T) -> WithContextRef<T> {
        WithContextRef::new(json!({ "@vocab": EDC_NAMESPACE }), inner)
    }

    pub fn odrl_context(inner: &'a T) -> WithContextRef<T> {
        WithContextRef::new(json!([ ODRL_CONTEXT,{ "@vocab": EDC_NAMESPACE }]), inner)
    }
}

impl<T> WithContext<T> {
    pub fn new(context: Value, inner: T) -> WithContext<T> {
        WithContext { context, inner }
    }
}
