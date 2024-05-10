use crate::components::{connectors::Connectors, footer::Footer};

#[derive(Debug, Default)]
pub struct AppModel {
    pub(crate) connectors: Connectors,
    pub(crate) footer: Footer,
    pub(crate) focus: AppFocus,
    pub(crate) footer_visible: bool,
}

impl AppModel {
    pub fn new(connectors: Connectors, footer: Footer) -> Self {
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
