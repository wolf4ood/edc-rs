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
pub struct Footer(FooterModel);

#[async_trait::async_trait]
impl Component for Footer {
    type Msg = FooterMsg;

    type Model = FooterModel;

    fn view(&mut self, f: &mut Frame, area: Rect) {
        let text_area = &mut self.0.area;
        text_area.set_block(Block::default().borders(Borders::all()));
        text_area.set_cursor_line_style(Style::default());
        text_area.set_placeholder_text("Enter command");
        f.render_widget(self.0.area.widget(), area)
    }

    async fn update(&mut self, msg: ComponentMsg<Self::Msg>) -> anyhow::Result<()> {
        match msg {
            ComponentMsg::Local(FooterMsg::AppendCommand(input)) => {
                self.0.area.input(input);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_event(
        &self,
        evt: crossterm::event::Event,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>> {
        let input: Input = evt.into();

        if input.key == Key::Esc {
            Ok(Some(ComponentMsg::Global(super::GlobalMsg::Esc)))
        } else {
            Ok(Some(ComponentMsg::Local(FooterMsg::AppendCommand(input))))
        }
    }

    fn init(_config: crate::config::Config) -> Self {
        Footer::default()
    }
}
