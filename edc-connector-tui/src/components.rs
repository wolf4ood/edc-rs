use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

use crate::{nav::Nav, types::connector::Connector};

pub mod connectors;
pub mod footer;
pub mod assets;

#[async_trait::async_trait]
pub trait Component {
    type Msg: Send;
    type Model: Send;

    fn view(model: &mut Self::Model, f: &mut Frame, rect: Rect);

    async fn update(
        model: &mut Self::Model,
        message: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>>;

    fn handle_event(
        model: &Self::Model,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>>;

    async fn forward_update<F, C>(
        model: &mut C::Model,
        msg: ComponentMsg<C::Msg>,
        mapper: F,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>>
    where
        F: Fn(C::Msg) -> Self::Msg + Send,
        C: Component + Sync + Send,
    {
        Ok(C::update(model, msg).await?.map(mapper))
    }

    fn forward_event<F, C>(
        model: &C::Model,
        evt: ComponentEvent,
        mapper: F,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>>
    where
        F: Fn(C::Msg) -> Self::Msg,
        C: Component,
    {
        Ok(C::handle_event(model, evt)?
            .into_iter()
            .map(|c| c.map(&mapper))
            .collect())
    }
}

#[derive(Debug)]
pub enum ComponentMsg<T> {
    Global(GlobalMsg),
    Shared(SharedMsg),
    Local(T),
}

#[derive(Debug, Default)]
pub struct ComponentReturn<T> {
    pub(crate) msg: Option<ComponentMsg<T>>,
    cmd: Option<()>,
}

#[derive(Debug, Clone)]
pub enum GlobalMsg {
    Quit,
    Esc,
    NavTo(Nav),
}

#[derive(Debug, Clone)]
pub enum SharedMsg {
    ChangeConnector(Connector),
}

impl<T> ComponentMsg<T> {
    pub fn map<M, F>(self, mapper: F) -> ComponentMsg<M>
    where
        F: FnOnce(T) -> M,
    {
        match self {
            ComponentMsg::Global(g) => ComponentMsg::Global(g),
            ComponentMsg::Local(msg) => ComponentMsg::Local(mapper(msg)),
            ComponentMsg::Shared(s) => ComponentMsg::Shared(s),
        }
    }
}

impl<T> ComponentReturn<T> {
    pub fn empty() -> ComponentReturn<T> {
        ComponentReturn {
            msg: None,
            cmd: None,
        }
    }

    pub fn map<M, F>(self, mapper: F) -> ComponentReturn<M>
    where
        F: FnOnce(T) -> M,
    {
        let msg = match self.msg {
            Some(ComponentMsg::Global(g)) => Some(ComponentMsg::Global(g)),
            Some(ComponentMsg::Local(msg)) => Some(ComponentMsg::Local(mapper(msg))),
            Some(ComponentMsg::Shared(s)) => Some(ComponentMsg::Shared(s)),
            _ => None,
        };

        ComponentReturn { msg, cmd: self.cmd }
    }
}

impl<T> From<T> for ComponentMsg<T> {
    fn from(value: T) -> Self {
        ComponentMsg::Local(value)
    }
}

impl<T> From<ComponentMsg<T>> for ComponentReturn<T> {
    fn from(value: ComponentMsg<T>) -> Self {
        ComponentReturn {
            msg: Some(value),
            cmd: None,
        }
    }
}

#[derive(Clone)]
pub enum ComponentEvent {
    Event(Event),
    Show,
    Hide,
    Tick,
}
