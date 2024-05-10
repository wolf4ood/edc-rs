use std::rc::Rc;
mod model;
mod msg;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    components::{connectors::Connectors, footer::Footer, Component, ComponentMsg, GlobalMsg},
    constants::{BANNER, HIGHLIGHT_COLOR},
};

use self::{
    model::{AppFocus, AppModel},
    msg::AppMsg,
};

pub struct App(AppModel);

#[async_trait::async_trait]
impl Component for App {
    type Msg = AppMsg;

    type Model = AppModel;

    fn view(&mut self, f: &mut Frame, rect: ratatui::prelude::Rect) {
        let main = self.main_layout(rect);
        f.render_widget(self.header(), main[0]);
        self.0.connectors.view(f, main[1]);
        self.0.footer.view(f, main[2])
    }

    async fn update(&mut self, msg: ComponentMsg<Self::Msg>) -> anyhow::Result<()> {
        match msg {
            ComponentMsg::Local(AppMsg::ConnectorsMsg(m)) => self.0.connectors.update(m.into()).await,
            ComponentMsg::Local(AppMsg::ShowFooter) => {
                self.0.footer_visible = true;
                self.0.focus = AppFocus::Footer;
                Ok(())
            }
            ComponentMsg::Local(AppMsg::FooterMsg(m)) => self.0.footer.update(m.into()).await,
            _ => Ok(())
        }
    }

    fn handle_event(&self, evt: Event) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>> {
        let msg = match self.0.focus {
            model::AppFocus::ConnectorList => {
                self.forward_event(evt.clone(), &self.0.connectors, AppMsg::ConnectorsMsg)?
            }
            model::AppFocus::Footer => {
                self.forward_event(evt.clone(), &self.0.footer, AppMsg::FooterMsg)?
            }
        };

        if msg.is_none() {
            if let Event::Key(key) = evt {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(self.handle_key(key));
                }
            }
        } else {
            return Ok(msg);
        }

        Ok(None)
    }

    fn init(config: crate::config::Config) -> Self {
        let connectors = Connectors::init(config.clone());
        let footer = Footer::init(config.clone());

        App(AppModel::new(connectors, footer))
    }
}

impl App {
    fn header(&self) -> Paragraph {
        let top_text = Text::from(BANNER).patch_style(Style::default().fg(HIGHLIGHT_COLOR));
        Paragraph::new(top_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Right)
            .block(Block::default())
    }

    fn main_layout(&self, rect: Rect) -> Rc<[Rect]> {
        let body_percentage = if self.0.footer_visible { 80 } else { 85 };
        let footer_percentage = 100 - body_percentage - 15;
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(body_percentage),
                    Constraint::Percentage(footer_percentage),
                ]
                .as_ref(),
            )
            .split(rect)
    }

    fn handle_key(&self, key: event::KeyEvent) -> Option<ComponentMsg<AppMsg>> {
        match key.code {
            KeyCode::Char('q') => Some(ComponentMsg::Global(GlobalMsg::Quit)),
            KeyCode::Char(':') => Some(ComponentMsg::Local(AppMsg::ShowFooter)),
            _ => None,
        }
    }
}
