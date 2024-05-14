use edc_connector_client::types::asset::Asset;

use crate::components::table::msg::TableMsg;

#[derive(Debug)]
pub enum AssetsMsg {
    FecthedAsset(Vec<Asset>),
    TableEvent(TableMsg),
}
