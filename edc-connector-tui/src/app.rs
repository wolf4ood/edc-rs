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
        assets::Assets, connectors::Connectors, footer::Footer, Component, ComponentEvent,
        ComponentMsg, ComponentReturn, GlobalMsg, SharedMsg,
    },
    constants::{BANNER, HIGHLIGHT_COLOR},
    nav::Nav,
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

        match &model.nav {
            crate::nav::Nav::ConnectorsList => Connectors::view(&mut model.connectors, f, main[1]),
            crate::nav::Nav::AssetsList => Assets::view(&mut model.assets, f, main[1]),
        }

        Footer::view(&mut model.footer, f, main[2]);
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<AppMsg>> {
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
                Ok(ComponentReturn::empty())
            }
            ComponentMsg::Local(AppMsg::FooterMsg(m)) => {
                Self::forward_update::<_, Footer>(&mut model.footer, m.into(), AppMsg::FooterMsg)
                    .await
            }
            ComponentMsg::Local(AppMsg::AssetsMsg(m)) => {
                Self::forward_update::<_, Assets>(&mut model.assets, m.into(), AppMsg::AssetsMsg)
                    .await
            }
            ComponentMsg::Global(GlobalMsg::Esc) => {
                model.footer_visible = false;
                model.focus = AppFocus::ConnectorList;
                Ok(ComponentReturn::empty())
            }
            ComponentMsg::Global(GlobalMsg::NavTo(Nav::ConnectorsList)) => {
                model.footer_visible = false;
                model.focus = AppFocus::ConnectorList;
                model.nav = Nav::ConnectorsList;
                Ok(ComponentReturn::empty())
            }
            ComponentMsg::Global(GlobalMsg::NavTo(Nav::AssetsList)) => {
                model.footer_visible = false;
                model.focus = AppFocus::Assets;
                model.nav = Nav::AssetsList;
                Ok(ComponentReturn::empty())
            }
            ComponentMsg::Shared(shared) => {
                match (&model.focus, &shared) {
                    (AppFocus::ConnectorList | AppFocus::Footer, _) => {
                        model.focus = AppFocus::Assets;
                        model.nav = Nav::AssetsList;
                    }
                    (AppFocus::Assets, _) => {}
                };

                Self::broadcast(model, shared).await

            }
            _ => Ok(ComponentReturn::empty()),
        }
    }

    fn handle_event(
        model: &Self::Model,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        let msg = match model.focus {
            AppFocus::ConnectorList => Self::forward_event::<_, Connectors>(
                &model.connectors,
                evt.clone(),
                AppMsg::ConnectorsMsg,
            )?,
            AppFocus::Footer => {
                Self::forward_event::<_, Footer>(&model.footer, evt.clone(), AppMsg::FooterMsg)?
            }
            AppFocus::Assets => {
                Self::forward_event::<_, Assets>(&model.assets, evt.clone(), AppMsg::AssetsMsg)?
            }
        };

        if msg.is_empty() {
            if let ComponentEvent::Event(Event::Key(key)) = evt {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(Self::handle_key(key));
                }
            }
        } else {
            return Ok(msg);
        }

        Ok(vec![])
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

    async fn broadcast(
        model: &mut AppModel,
        shared: SharedMsg,
    ) -> anyhow::Result<ComponentReturn<AppMsg>> {
        let events = Self::forward_update::<_, Footer>(
            &mut model.footer,
            ComponentMsg::Shared(shared.clone()),
            AppMsg::FooterMsg,
        )
        .await?;

        let events = Self::forward_update::<_, Assets>(
            &mut model.assets,
            ComponentMsg::Shared(shared),
            AppMsg::AssetsMsg,
        )
        .await?;

        Ok(ComponentReturn::empty())
    }

    fn handle_key(key: event::KeyEvent) -> Vec<ComponentMsg<AppMsg>> {
        match key.code {
            KeyCode::Char(':') => vec![(ComponentMsg::Local(AppMsg::ShowFooter))],
            _ => vec![],
        }
    }
}
