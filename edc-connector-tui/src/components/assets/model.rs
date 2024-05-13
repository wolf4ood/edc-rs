use std::fmt::Debug;

use edc_connector_client::EdcConnectorClient;
use ratatui::widgets::TableState;

#[derive(Default)]
pub struct AssetsModel {
    pub(crate) table_state: TableState,
    pub(crate) client: Option<EdcConnectorClient>,
}

impl Debug for AssetsModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssetsModel")
            .field("table_state", &self.table_state)
            .field("client", &"EdcConnectorClient")
            .finish()
    }
}
