use ratatui::widgets::TableState;

use crate::types::connector::Connector;

#[derive(Debug, Default)]
pub struct ConnectorsModel {
    pub(crate) table_state: TableState,
    pub(crate) connectors: Vec<Connector>,
}

impl ConnectorsModel {
    pub fn new(connectors: Vec<Connector>) -> Self {
        Self {
            table_state: TableState::default().with_selected(0),
            connectors,
        }
    }
}
