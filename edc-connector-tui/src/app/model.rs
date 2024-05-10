use edc_connector_client::EdcConnectorClient;

use crate::{
    components::{connectors::model::ConnectorsModel, footer::model::FooterModel},
    config::Config,
    types::connector::Connector,
};

#[derive(Debug, Default)]
pub struct AppModel {
    pub(crate) connectors: ConnectorsModel,
    pub(crate) footer: FooterModel,
    pub(crate) focus: AppFocus,
    pub(crate) footer_visible: bool,
}

impl AppModel {
    pub fn init(cfg: Config) -> AppModel {
        let connectors = cfg
            .connectors
            .into_iter()
            .map(|cfg| {
                let client = EdcConnectorClient::builder()
                    .management_url(cfg.address())
                    .build()
                    .unwrap();
                Connector::new(cfg, client)
            })
            .collect();
        let connectors = ConnectorsModel::new(connectors);
        let footer = FooterModel::default();

        AppModel::new(connectors, footer)
    }

    pub fn new(connectors: ConnectorsModel, footer: FooterModel) -> Self {
        Self {
            connectors,
            footer,
            focus: AppFocus::ConnectorList,
            footer_visible: false,
        }
    }
}

#[derive(Debug, Default)]
pub enum AppFocus {
    #[default]
    ConnectorList,
    Footer,
}
