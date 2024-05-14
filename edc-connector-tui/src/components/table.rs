use std::marker::PhantomData;

use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{block::Title, Block, BorderType, Borders, Row, Table},
    Frame,
};
pub mod model;
pub mod msg;

use self::{model::TableModel, msg::TableMsg};

use super::{Component, ComponentEvent, ComponentMsg, ComponentReturn};

#[derive(Debug, Default)]
pub struct UiTable<T: TableEntry>(PhantomData<T>);

pub trait TableEntry {
    fn row(&self) -> Row;
    fn headers() -> Row<'static>;
}

#[async_trait::async_trait]
impl<T: TableEntry + Send> Component for UiTable<T> {
    type Msg = TableMsg;
    type Model = TableModel<T>;
    fn view(model: &mut Self::Model, f: &mut Frame, area: Rect) {
        let styled_text =
            Span::styled(format!(" {} ", model.name), Style::default().fg(Color::Red));
        let block = Block::default()
            .title(Title::from(styled_text).alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let rows = model
            .elements
            .iter()
            .map(TableEntry::row)
            .collect::<Vec<_>>();

        let table = Table::default()
            .rows(rows)
            .header(T::headers())
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(table, area, &mut model.table_state);
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        match msg {
            ComponentMsg::Global(_) => todo!(),
            ComponentMsg::Shared(_) => todo!(),
            ComponentMsg::Local(TableMsg::MoveDown) => Self::move_down(model),
            ComponentMsg::Local(TableMsg::MoveUp) => Self::move_up(model),
        };

        Ok(ComponentReturn::empty())
    }

    fn handle_event(
        model: &Self::Model,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        match evt {
            ComponentEvent::Event(Event::Key(key)) => Ok(Self::handle_key(key)),
            _ => Ok(vec![]),
        }
    }
}

impl<T: TableEntry> UiTable<T> {
    fn handle_key(key: KeyEvent) -> Vec<ComponentMsg<TableMsg>> {
        match key.code {
            KeyCode::Char('j') => vec![(ComponentMsg::Local(TableMsg::MoveDown))],
            KeyCode::Char('k') => vec![(ComponentMsg::Local(TableMsg::MoveUp))],
            _ => vec![],
        }
    }

    fn move_up(model: &mut TableModel<T>) {
        let new_pos = match model.table_state.selected() {
            Some(i) if i == 0 => model.elements.len() - 1,
            Some(i) => i - 1,
            None => 0,
        };
        model.table_state.select(Some(new_pos))
    }

    fn move_down(model: &mut TableModel<T>) {
        let new_pos = match model.table_state.selected() {
            Some(i) if i == model.elements.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        model.table_state.select(Some(new_pos))
    }
}
