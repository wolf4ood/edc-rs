use std::fmt::Debug;

use crate::{components::table::model::TableModel, types::connector::Connector};

use super::AssetEntry;

#[derive(Debug)]
pub struct AssetsModel {
    pub(crate) table: TableModel<AssetEntry>,
    pub(crate) client: Option<Connector>,
}

impl Default for AssetsModel {
    fn default() -> Self {
        Self {
            table: TableModel::new("Assets".to_string()),
            client: Default::default(),
        }
    }
}
