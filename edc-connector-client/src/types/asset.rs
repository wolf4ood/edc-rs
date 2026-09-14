use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::error::ConversionError;

use super::{
    data_address::DataAddress,
    properties::{FromValue, Properties, PropertyValue, ToValue},
};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[builder(field)]
    properties: Properties,
    #[builder(field)]
    #[serde(default = "Default::default")]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(default = "Asset".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated]
    data_address: Option<DataAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dataplane_metadata: Option<DataplaneMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewAsset {
    #[builder(field)]
    properties: Properties,
    #[builder(field)]
    #[serde(default = "Default::default")]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[builder(default = "Asset".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated]
    data_address: Option<DataAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dataplane_metadata: Option<DataplaneMetadata>,
}

/// Metadata describing the data plane requirements of an [`Asset`].
///
/// It replaces the deprecated `dataAddress` on the asset: `labels` and
/// `profiles` are used to select a data plane, while `properties` is a free
/// form JSON object handed over to the selected data plane.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DataplaneMetadata {
    #[builder(field)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    labels: Vec<String>,
    #[builder(field)]
    #[serde(default, skip_serializing_if = "Properties::is_empty")]
    properties: Properties,
    #[builder(field)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    profiles: Vec<String>,
    #[builder(default = DATAPLANE_METADATA_TYPE.to_string())]
    #[serde(rename = "@type", default = "default_dataplane_metadata_type")]
    ty: String,
}

const DATAPLANE_METADATA_TYPE: &str = "DataplaneMetadata";

fn default_dataplane_metadata_type() -> String {
    DATAPLANE_METADATA_TYPE.to_string()
}

impl Asset {
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

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn private_properties(&self) -> &Properties {
        &self.private_properties
    }

    #[deprecated]
    pub fn data_address(&self) -> Option<&DataAddress> {
        self.data_address.as_ref()
    }

    pub fn dataplane_metadata(&self) -> Option<&DataplaneMetadata> {
        self.dataplane_metadata.as_ref()
    }
}

impl NewAsset {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn private_properties(&self) -> &Properties {
        &self.private_properties
    }

    #[deprecated]
    pub fn data_address(&self) -> Option<&DataAddress> {
        self.data_address.as_ref()
    }

    pub fn dataplane_metadata(&self) -> Option<&DataplaneMetadata> {
        self.dataplane_metadata.as_ref()
    }
}

impl DataplaneMetadata {
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    pub fn profiles(&self) -> &[String] {
        &self.profiles
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.properties.get(property)
    }

    pub fn raw_property(&self, property: &str) -> Option<&PropertyValue> {
        self.properties.get_raw(property)
    }
}

impl<S: asset_builder::State> AssetBuilder<S> {
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
}

impl<S: new_asset_builder::State> NewAssetBuilder<S> {
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
}

impl<S: dataplane_metadata_builder::State> DataplaneMetadataBuilder<S> {
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    pub fn labels<I, T>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.labels.extend(labels.into_iter().map(Into::into));
        self
    }

    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profiles.push(profile.into());
        self
    }

    pub fn profiles<I, T>(mut self, profiles: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.profiles.extend(profiles.into_iter().map(Into::into));
        self
    }

    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::{Asset, DataplaneMetadata, NewAsset};

    #[test]
    fn should_serialize_a_new_asset_with_dataplane_metadata() {
        let asset = NewAsset::builder()
            .id("asset-id")
            .property("foo", "bar")
            .dataplane_metadata(
                DataplaneMetadata::builder()
                    .label("label1")
                    .labels(["label2"])
                    .profile("http-profile")
                    .property("key", "value")
                    .build(),
            )
            .build();

        assert_eq!(
            serde_json::to_value(&asset).unwrap(),
            json!({
                "@id": "asset-id",
                "@type": "Asset",
                "properties": { "foo": "bar" },
                "privateProperties": {},
                "dataplaneMetadata": {
                    "@type": "DataplaneMetadata",
                    "labels": ["label1", "label2"],
                    "properties": { "key": "value" },
                    "profiles": ["http-profile"]
                }
            })
        );
    }

    #[test]
    fn should_omit_empty_dataplane_metadata_fields() {
        let metadata = DataplaneMetadata::builder().profile("http-profile").build();

        assert_eq!(
            serde_json::to_value(&metadata).unwrap(),
            json!({
                "@type": "DataplaneMetadata",
                "profiles": ["http-profile"]
            })
        );
    }

    #[test]
    fn should_deserialize_an_asset_with_dataplane_metadata() {
        let asset: Asset = serde_json::from_value(json!({
            "@id": "asset-id",
            "@type": "Asset",
            "properties": { "foo": "bar" },
            "dataplaneMetadata": {
                "@type": "DataplaneMetadata",
                "labels": ["label1"],
                "properties": {
                    "key": "value",
                    "complex": { "nested": "value" }
                },
                "profiles": ["http-profile"]
            }
        }))
        .unwrap();

        let metadata = asset.dataplane_metadata().unwrap();

        assert_eq!(metadata.labels(), ["label1"]);
        assert_eq!(metadata.profiles(), ["http-profile"]);
        assert_eq!(
            Some("value".to_string()),
            metadata.property::<String>("key").unwrap()
        );
        assert_eq!(
            Some(&json!({ "nested": "value" })),
            metadata.raw_property("complex").map(|value| &value.0)
        );
    }

    #[test]
    fn should_deserialize_dataplane_metadata_with_missing_fields() {
        let asset: Asset = serde_json::from_value(json!({
            "@id": "asset-id",
            "@type": "Asset",
            "properties": {},
            "dataplaneMetadata": {
                "profiles": "single-profile"
            }
        }))
        .unwrap();

        let metadata = asset.dataplane_metadata().unwrap();

        assert!(metadata.labels().is_empty());
        assert!(metadata.properties().is_empty());
        assert_eq!(metadata.profiles(), ["single-profile"]);
    }

    #[test]
    fn should_deserialize_an_asset_without_dataplane_metadata() {
        let asset: Asset = serde_json::from_value(json!({
            "@id": "asset-id",
            "@type": "Asset",
            "properties": {}
        }))
        .unwrap();

        assert!(asset.dataplane_metadata().is_none());
    }
}
