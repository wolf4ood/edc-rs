mod conversion;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ConversionError;

pub use self::conversion::{FromValue, ToValue};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Properties(HashMap<String, PropertyValue>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PropertyValue(pub(crate) Value);

impl Properties {
    pub fn get<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.0
            .get(property)
            .map(PropertyValue::try_from)
            .transpose()
    }

    pub fn get_raw(&self, property: &str) -> Option<&PropertyValue> {
        self.0.get(property)
    }

    pub(crate) fn set<T>(&mut self, property: &str, value: T)
    where
        T: ToValue,
    {
        self.0
            .insert(property.to_string(), PropertyValue(value.into_value()));
    }

    pub fn contains(&self, property: &str) -> bool {
        self.0.contains_key(property)
    }
}

impl PropertyValue {
    pub fn try_from<T>(&self) -> Result<T, ConversionError>
    where
        T: FromValue,
    {
        T::try_from(&self.0)
    }
}
