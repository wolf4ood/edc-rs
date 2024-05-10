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
pub struct Connectors;

#[async_trait::async_trait]
impl Component for Connectors {
    type Msg = ConnectorsMsg;

    type Model = ConnectorsModel;

    fn view(model: &mut Self::Model, f: &mut Frame, area: Rect) {
        let styled_text = Span::styled(" Connectors ", Style::default().fg(Color::Red));
        let block = Block::default()
            .title(Title::from(styled_text).alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let rows: Vec<_> = model
            .connectors
            .iter()
            .map(|connector| {
                Row::new(vec![
                    connector.config().name(),
                    connector.config().address(),
                ])
            })
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

        f.render_stateful_widget(table, area, &mut model.table_state);
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>> {
        match msg {
            ComponentMsg::Local(ConnectorsMsg::MoveUp) => Self::move_up(model),
            ComponentMsg::Local(ConnectorsMsg::MoveDown) => Self::move_down(model),
            ComponentMsg::Local(ConnectorsMsg::SelectCurrent) => {
                if let Some(idx) = model.table_state.selected() {
                    if let Some(connector) = model.connectors.get(idx) {
                        return Ok(Some(ComponentMsg::Shared(
                            super::SharedMsg::ChangeConnector(connector.clone()),
                        )));
                    }
                }
            }
            _ => {}
        };

        Ok(None)
    }

    fn handle_event(
        _model: &Self::Model,
        evt: crossterm::event::Event,
    ) -> anyhow::Result<Option<super::ComponentMsg<Self::Msg>>> {
        match evt {
            crossterm::event::Event::Key(key) => Ok(Self::handle_key(key)),
            _ => Ok(None),
        }
    }
}

impl Connectors {
    fn handle_key(key: KeyEvent) -> Option<ComponentMsg<ConnectorsMsg>> {
        match key.code {
            KeyCode::Char('j') => Some(ComponentMsg::Local(ConnectorsMsg::MoveDown)),
            KeyCode::Char('k') => Some(ComponentMsg::Local(ConnectorsMsg::MoveUp)),
            KeyCode::Enter => Some(ComponentMsg::Local(ConnectorsMsg::SelectCurrent)),
            _ => None,
        }
    }

    fn move_up(model: &mut ConnectorsModel) {
        let new_pos = match model.table_state.selected() {
            Some(i) if i == 0 => model.connectors.len() - 1,
            Some(i) => i - 1,
            None => 0,
        };
        model.table_state.select(Some(new_pos))
    }

    fn move_down(model: &mut ConnectorsModel) {
        let new_pos = match model.table_state.selected() {
            Some(i) if i == model.connectors.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        model.table_state.select(Some(new_pos))
    }
}
