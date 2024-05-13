use self::{model::FooterModel, msg::FooterMsg};
use super::{Component, ComponentEvent, ComponentMsg, ComponentReturn};
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
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        match msg {
            ComponentMsg::Local(FooterMsg::AppendCommand(input)) => {
                model.area.input(input);
            }
            _ => {}
        };
        Ok(ComponentReturn::empty())
    }

    fn handle_event(
        model: &Self::Model,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        if let ComponentEvent::Event(evt) = evt {
            let input: Input = evt.into();

            let current = &model.area.lines()[0];
            match input.key {
                Key::Char('q') if current.is_empty() => Ok(vec![
                    ComponentMsg::Local(FooterMsg::AppendCommand(input)),
                    ComponentMsg::Local(FooterMsg::AppendCommand(Input {
                        key: Key::Char('!'),
                        ..Default::default()
                    })),
                ]),
                Key::Enter if current == "q!" => {
                    Ok(vec![ComponentMsg::Global(super::GlobalMsg::Quit)])
                }
                Key::Enter => Ok(vec![ComponentMsg::Global(super::GlobalMsg::NavTo(
                    model.area.lines()[0].parse()?,
                ))]),
                Key::Esc => Ok(vec![ComponentMsg::Global(super::GlobalMsg::Esc)]),
                _ => Ok(vec![ComponentMsg::Local(FooterMsg::AppendCommand(input))]),
            }
        } else {
            Ok(vec![])
        }
    }
}
