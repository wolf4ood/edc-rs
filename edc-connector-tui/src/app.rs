use std::rc::Rc;
pub mod model;
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
    components::{
        connectors::Connectors, footer::Footer, Component, ComponentMsg, GlobalMsg, SharedMsg,
    },
    constants::{BANNER, HIGHLIGHT_COLOR},
};

use self::{
    model::{AppFocus, AppModel},
    msg::AppMsg,
};

pub struct App;

#[async_trait::async_trait]
impl Component for App {
    type Msg = AppMsg;
    type Model = AppModel;

    fn view(model: &mut Self::Model, f: &mut Frame, rect: Rect) {
        let main = Self::main_layout(model, rect);
        f.render_widget(Self::header(), main[0]);
        Connectors::view(&mut model.connectors, f, main[1]);
        Footer::view(&mut model.footer, f, main[2]);
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<Option<ComponentMsg<AppMsg>>> {
        match msg {
            ComponentMsg::Local(AppMsg::ConnectorsMsg(m)) => {
                Self::forward_update::<_, Connectors>(
                    &mut model.connectors,
                    m.into(),
                    AppMsg::ConnectorsMsg,
                )
                .await
            }
            ComponentMsg::Local(AppMsg::ShowFooter) => {
                model.footer_visible = true;
                model.focus = AppFocus::Footer;
                Ok(None)
            }
            ComponentMsg::Local(AppMsg::FooterMsg(m)) => {
                Self::forward_update::<_, Footer>(&mut model.footer, m.into(), AppMsg::FooterMsg)
                    .await
            }
            ComponentMsg::Global(GlobalMsg::Esc) => {
                model.footer_visible = false;
                model.focus = AppFocus::ConnectorList;
                Ok(None)
            }
            ComponentMsg::Shared(SharedMsg::ChangeConnector(connector)) => {
                println!("New connector {}", connector.config().name());
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn handle_event(
        model: &Self::Model,
        evt: Event,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>> {
        let msg = match model.focus {
            model::AppFocus::ConnectorList => Self::forward_event::<_, Connectors>(
                &model.connectors,
                evt.clone(),
                AppMsg::ConnectorsMsg,
            )?,
            model::AppFocus::Footer => {
                Self::forward_event::<_, Footer>(&model.footer, evt.clone(), AppMsg::FooterMsg)?
            }
        };

        if msg.is_none() {
            if let Event::Key(key) = evt {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(Self::handle_key(key));
                }
            }
        } else {
            return Ok(msg);
        }

        Ok(None)
    }
}

impl App {
    fn header() -> Paragraph<'static> {
        let top_text = Text::from(BANNER).patch_style(Style::default().fg(HIGHLIGHT_COLOR));
        Paragraph::new(top_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Right)
            .block(Block::default())
    }

    fn main_layout(model: &AppModel, rect: Rect) -> Rc<[Rect]> {
        let body_percentage = if model.footer_visible { 80 } else { 85 };
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

    fn handle_key(key: event::KeyEvent) -> Option<ComponentMsg<AppMsg>> {
        match key.code {
            KeyCode::Char('q') => Some(ComponentMsg::Global(GlobalMsg::Quit)),
            KeyCode::Char(':') => Some(ComponentMsg::Local(AppMsg::ShowFooter)),
            _ => None,
        }
    }
}
