use std::fmt::Debug;

use edc_connector_client::EdcConnectorClient;

use crate::config::ConnectorConfig;

#[derive(Clone)]
pub struct Connector {
    config: ConnectorConfig,
    client: EdcConnectorClient,
}

impl Connector {
    pub fn new(config: ConnectorConfig, client: EdcConnectorClient) -> Self {
        Self { config, client }
    }

    pub fn config(&self) -> &ConnectorConfig {
        &self.config
    }

    pub fn client(&self) -> &EdcConnectorClient {
        &self.client
    }
}

impl Debug for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connctor")
            .field("config", &self.config)
            .finish()
    }
}
