use serde::{Deserialize, Serialize};

use crate::{error::BuilderError, ConversionError};

use super::properties::{FromValue, Properties, ToValue};

#[derive(Debug, Deserialize, Serialize)]
pub struct DataAddress(Properties);

impl DataAddress {
    pub fn builder() -> DataAddressBuilder {
        DataAddressBuilder::default()
    }

    pub fn property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.0.get(property)
    }
}

#[derive(Default)]
pub struct DataAddressBuilder(Properties);

impl DataAddressBuilder {
    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.0.set(property, value);
        self
    }

    pub fn kind(mut self, kind: &str) -> Self {
        self.0.set("type", kind);

        self
    }

    pub fn build(self) -> Result<DataAddress, BuilderError> {
        if self.0.contains("type") {
            Ok(DataAddress(self.0))
        } else {
            Err(BuilderError::missing_property("type"))
        }
    }
}
