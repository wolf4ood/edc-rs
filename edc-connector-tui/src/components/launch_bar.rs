use self::msg::LaunchBarMsg;
use super::{Action, Component, ComponentEvent, ComponentMsg, ComponentReturn};
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Widget},
    Frame,
};
use tui_textarea::{Input, Key, TextArea};
pub mod msg;

pub static PROMPT: &str = "$ ";

#[derive(Debug)]
pub struct LaunchBar {
    pub(crate) area: TextArea<'static>,
}
impl Default for LaunchBar {
    fn default() -> Self {
        let mut area = TextArea::default();
        area.insert_str(PROMPT);
        Self { area }
    }
}

#[async_trait::async_trait]
impl Component for LaunchBar {
    type Msg = LaunchBarMsg;
    type Props = ();

    fn view(&mut self, f: &mut Frame, rect: Rect) {
        let text_area = &mut self.area;
        text_area.set_block(Block::default().borders(Borders::all()));
        text_area.set_cursor_line_style(Style::default());
        text_area.set_placeholder_text("Enter command");

        self.area.render(rect, f.buffer_mut());
    }

    async fn update(
        &mut self,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        match msg.to_owned() {
            LaunchBarMsg::AppendCommand(input) => {
                self.area.input(input);
                Ok(ComponentReturn::empty())
            }
            LaunchBarMsg::Quit => Ok(ComponentReturn::action(Action::Quit)),
            LaunchBarMsg::Esc => Ok(ComponentReturn::action(Action::Esc)),
            LaunchBarMsg::NavTo(nav) => Ok(ComponentReturn::action(Action::NavTo(nav))),
        }
    }

    fn handle_event(
        &mut self,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        match evt {
            ComponentEvent::Event(evt) => {
                let input: Input = evt.into();

                let current = &self.area.lines()[0].replacen(PROMPT, "", 1);

                match input.key {
                    Key::Backspace if current.is_empty() => Ok(vec![]),
                    Key::Char('q') if current.is_empty() => Ok(vec![
                        LaunchBarMsg::AppendCommand(input).into(),
                        LaunchBarMsg::AppendCommand(Input {
                            key: Key::Char('!'),
                            ..Default::default()
                        })
                        .into(),
                    ]),
                    Key::Enter if current == "q!" => Ok(vec![LaunchBarMsg::Quit.into()]),
                    Key::Enter if !current.is_empty() => {
                        Ok(vec![LaunchBarMsg::NavTo(current.parse()?).into()])
                    }
                    Key::Esc => Ok(vec![LaunchBarMsg::Esc.into()]),
                    _ => Ok(vec![LaunchBarMsg::AppendCommand(input).into()]),
                }
            }
        }
    }
}

impl LaunchBar {
    pub fn clear(&mut self) {
        self.area.move_cursor(tui_textarea::CursorMove::Head);
        self.area.delete_line_by_end();
        self.area.insert_str(PROMPT);
    }
}
