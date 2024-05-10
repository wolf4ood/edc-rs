use self::{model::FooterModel, msg::FooterMsg};
use super::{Component, ComponentMsg};
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders},
    Frame,
};
use tui_textarea::{Input, Key};
pub mod model;
pub mod msg;

#[derive(Default, Debug)]
pub struct Footer;

#[async_trait::async_trait]
impl Component for Footer {
    type Msg = FooterMsg;

    type Model = FooterModel;

    fn view(model: &mut Self::Model, f: &mut Frame, area: Rect) {
        let text_area = &mut model.area;
        text_area.set_block(Block::default().borders(Borders::all()));
        text_area.set_cursor_line_style(Style::default());
        text_area.set_placeholder_text("Enter command");
        f.render_widget(model.area.widget(), area)
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>> {
        match msg {
            ComponentMsg::Local(FooterMsg::AppendCommand(input)) => {
                model.area.input(input);
            }
            _ => {}
        };
        Ok(None)
    }

    fn handle_event(
        _model: &Self::Model,
        evt: crossterm::event::Event,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>> {
        let input: Input = evt.into();

        if input.key == Key::Esc {
            Ok(Some(ComponentMsg::Global(super::GlobalMsg::Esc)))
        } else {
            Ok(Some(ComponentMsg::Local(FooterMsg::AppendCommand(input))))
        }
    }
}
