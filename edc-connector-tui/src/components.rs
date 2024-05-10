use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

use crate::config::Config;

pub mod connectors;
pub mod footer;

#[async_trait::async_trait]
pub trait Component {
    type Msg;
    type Model;

    fn init(config: Config) -> Self;
    fn view(&mut self, f: &mut Frame, rect: Rect);

    async fn update(&mut self, message: ComponentMsg<Self::Msg>) -> anyhow::Result<()>;

    fn handle_event(&self, evt: Event) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>>;

    fn forward_event<F, C>(
        &self,
        evt: Event,
        component: &C,
        mapper: F,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>>
    where
        F: FnOnce(C::Msg) -> Self::Msg,
        C: Component,
    {
        match component.handle_event(evt)? {
            Some(c) => Ok(Some(c.map(mapper))),
            None => Ok(None),
        }
    }
}

#[derive(Debug)]
pub enum ComponentMsg<T> {
    Global(GlobalMsg),
    Local(T),
}

#[derive(Debug, Clone)]
pub enum GlobalMsg {
    Quit,
    Esc,
}

impl<T> ComponentMsg<T> {
    pub fn map<M, F>(self, mapper: F) -> ComponentMsg<M>
    where
        F: FnOnce(T) -> M,
    {
        match self {
            ComponentMsg::Global(g) => ComponentMsg::Global(g),
            ComponentMsg::Local(msg) => ComponentMsg::Local(mapper(msg)),
        }
    }
}

impl<T> From<T> for ComponentMsg<T> {
    fn from(value: T) -> Self {
        ComponentMsg::Local(value)
    }
}
