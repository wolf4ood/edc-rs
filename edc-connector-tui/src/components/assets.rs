use self::{model::AssetsModel, msg::AssetsMsg};

use super::{Component, ComponentEvent, ComponentMsg, ComponentReturn};
use ratatui::{
    layout::{Alignment, Rect}, style::{Color, Modifier, Style}, text::Span, widgets::{block::Title, Block, BorderType, Borders, Row, Table}, Frame
};
pub mod model;
pub mod msg;

#[derive(Default, Debug)]
pub struct Assets;

#[async_trait::async_trait]
impl Component for Assets {
    type Msg = AssetsMsg;

    type Model = AssetsModel;

    fn view(model: &mut Self::Model, f: &mut Frame, area: Rect) {

        let styled_text = Span::styled(" Assets ", Style::default().fg(Color::Red));
        let block = Block::default()
            .title(Title::from(styled_text).alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);


        let table = Table::default()
            .header(Row::new(vec!["Id", "Properties", "Private Properties", "Data Address"]))
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(table, area, &mut model.table_state);
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        Ok(ComponentReturn::empty())
    }

    fn handle_event(
        model: &Self::Model,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        Ok(vec![])
    }
}
