use crate::components::resources::Field;

use super::{
    resources::{msg::ResourcesMsg, DrawableResource, FieldValue, ResourcesComponent},
    table::TableEntry,
};
use edc_connector_client::types::asset::Asset;
use ratatui::widgets::Row;

pub type AssetsMsg = ResourcesMsg<AssetEntry>;
pub type AssetsComponent = ResourcesComponent<AssetEntry>;

#[derive(Debug, Clone)]
pub struct AssetEntry(Asset);

impl AssetEntry {
    pub fn new(asset: Asset) -> AssetEntry {
        AssetEntry(asset)
    }
}

impl TableEntry for AssetEntry {
    fn row(&self) -> Row {
        let properties = serde_json::to_string(self.0.properties()).unwrap();
        let private_properties = serde_json::to_string(self.0.private_properties()).unwrap();
        let data_address = serde_json::to_string(self.0.data_address()).unwrap();
        Row::new(vec![
            self.0.id().to_string(),
            properties,
            private_properties,
            data_address,
        ])
    }

    fn headers() -> Row<'static> {
        Row::new(vec![
            "ID",
            "PROPERTIES",
            "PRIVATE PROPERTIES",
            "DATA ADDRESS",
        ])
    }
}

impl DrawableResource for AssetEntry {
    fn id(&self) -> &str {
        self.0.id()
    }

    fn title() -> &'static str {
        "Assets"
    }

    fn fields(&self) -> Vec<super::resources::Field> {
        let mut fields = vec![Field::new(
            "id".to_string(),
            super::resources::FieldValue::Str(self.0.id().to_string()),
        )];

        let properties = self.0.properties();

        fields.push(Field::new(
            "properties".to_string(),
            FieldValue::Json(serde_json::to_string(&properties).unwrap()),
        ));

        fields
    }
}
