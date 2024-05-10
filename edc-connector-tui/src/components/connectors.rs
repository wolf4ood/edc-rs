use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{block::Title, Block, BorderType, Borders, Row, Table},
    Frame,
};

use self::{model::ConnectorsModel, msg::ConnectorsMsg};

use super::{Component, ComponentMsg};

pub mod model;
pub mod msg;

#[derive(Debug, Default)]
pub struct Connectors {
    model: model::ConnectorsModel,
}

#[async_trait::async_trait]
impl Component for Connectors {
    type Msg = ConnectorsMsg;

    type Model = ConnectorsModel;

    fn view(&mut self, f: &mut Frame, area: Rect) {
        let styled_text = Span::styled(" Connectors ", Style::default().fg(Color::Red));
        let block = Block::default()
            .title(Title::from(styled_text).alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let rows: Vec<_> = self
            .model
            .connectors
            .iter()
            .map(|connector| Row::new(vec![connector.name(), connector.address()]))
            .collect();

        let widths = [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ];
        let table = Table::new(rows, widths)
            .header(Row::new(vec!["Name", "Address", "Status"]))
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(table, area, &mut self.model.table_state);
    }

    async fn update(&mut self, msg: ComponentMsg<Self::Msg>) -> anyhow::Result<()> {
        match msg {
            ComponentMsg::Local(ConnectorsMsg::MoveUp) => self.move_up(),
            ComponentMsg::Local(ConnectorsMsg::MoveDown) => self.move_down(),
            _ => {}
        };

        Ok(())
    }

    fn handle_event(
        &self,
        evt: crossterm::event::Event,
    ) -> anyhow::Result<Option<super::ComponentMsg<Self::Msg>>> {
        match evt {
            crossterm::event::Event::Key(key) => Ok(self.handle_key(key)),
            _ => Ok(None),
        }
    }

    fn init(config: crate::config::Config) -> Self {
        Connectors {
            model: ConnectorsModel::new(config.connectors),
        }
    }
}

impl Connectors {
    fn handle_key(&self, key: KeyEvent) -> Option<ComponentMsg<ConnectorsMsg>> {
        match key.code {
            KeyCode::Char('j') => Some(ComponentMsg::Local(ConnectorsMsg::MoveDown)),
            KeyCode::Char('k') => Some(ComponentMsg::Local(ConnectorsMsg::MoveUp)),
            _ => None,
        }
    }

    fn move_up(&mut self) {
        let new_pos = match self.model.table_state.selected() {
            Some(i) if i == 0 => self.model.connectors.len() - 1,
            Some(i) => i - 1,
            None => 0,
        };
        self.model.table_state.select(Some(new_pos))
    }

    fn move_down(&mut self) {
        let new_pos = match self.model.table_state.selected() {
            Some(i) if i == self.model.connectors.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        self.model.table_state.select(Some(new_pos))
    }
}
