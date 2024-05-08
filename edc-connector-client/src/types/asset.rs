use serde::{Deserialize, Serialize};

use crate::error::{BuilderError, ConversionError};

use super::{
    data_address::DataAddress,
    properties::{FromValue, Properties, PropertyValue, ToValue},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[serde(rename = "@id")]
    id: String,
    properties: Properties,
    #[serde(default = "Default::default")]
    private_properties: Properties,
    data_address: DataAddress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAsset {
    #[serde(rename = "@id")]
    id: Option<String>,
    properties: Properties,
    #[serde(default = "Default::default")]
    private_properties: Properties,
    data_address: DataAddress,
}

impl NewAsset {
    pub fn builder() -> NewAssetBuilder {
        NewAssetBuilder::default()
    }
}

impl Asset {
    pub fn builder() -> AssetBuider {
        AssetBuider::default()
    }

    pub fn property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.properties.get(property)
    }

    pub fn raw_property(&self, property: &str) -> Option<&PropertyValue>
where {
        self.properties.get_raw(property)
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Default)]
pub struct AssetBuider {
    id: Option<String>,
    properties: Properties,
    private_properties: Properties,
    data_address: Option<DataAddress>,
}

impl AssetBuider {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }

    pub fn data_address(mut self, data_address: DataAddress) -> Self {
        self.data_address = Some(data_address);
        self
    }

    pub fn build(self) -> Result<Asset, BuilderError> {
        Ok(Asset {
            id: self
                .id
                .ok_or_else(|| BuilderError::missing_property("id"))?,
            properties: self.properties,
            private_properties: self.private_properties,
            data_address: self
                .data_address
                .ok_or_else(|| BuilderError::missing_property("data_address"))?,
        })
    }
}

#[derive(Default)]
pub struct NewAssetBuilder {
    id: Option<String>,
    properties: Properties,
    private_properties: Properties,
    data_address: Option<DataAddress>,
}

impl NewAssetBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }

    pub fn data_address(mut self, data_address: DataAddress) -> Self {
        self.data_address = Some(data_address);
        self
    }

    pub fn build(self) -> Result<NewAsset, BuilderError> {
        Ok(NewAsset {
            id: self.id,
            properties: self.properties,
            private_properties: self.private_properties,
            data_address: self
                .data_address
                .ok_or_else(|| BuilderError::missing_property("data_address"))?,
        })
    }
}
