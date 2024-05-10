use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

pub mod connectors;
pub mod footer;

#[async_trait::async_trait]
pub trait Component {
    type Msg: Send;
    type Model: Send;

    fn view(model: &mut Self::Model, f: &mut Frame, rect: Rect);

    async fn update(
        model: &mut Self::Model,
        message: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>>;

    fn handle_event(
        model: &Self::Model,
        evt: Event,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>>;

    async fn forward_update<F, C>(
        model: &mut C::Model,
        msg: ComponentMsg<C::Msg>,
        mapper: F,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>>
    where
        F: FnOnce(C::Msg) -> Self::Msg + Send,
        C: Component + Sync + Send,
    {
        match C::update(model, msg).await? {
            Some(c) => Ok(Some(c.map(mapper))),
            None => Ok(None),
        }
    }

    fn forward_event<F, C>(
        model: &C::Model,
        evt: Event,
        mapper: F,
    ) -> anyhow::Result<Option<ComponentMsg<Self::Msg>>>
    where
        F: FnOnce(C::Msg) -> Self::Msg,
        C: Component,
    {
        match C::handle_event(model, evt)? {
            Some(c) => Ok(Some(c.map(mapper))),
            None => Ok(None),
        }
    }
}

#[derive(Debug)]
pub enum ComponentMsg<T> {
    Global(GlobalMsg),
    Shared(SharedMsg),
    Local(T),
}

#[derive(Debug, Clone)]
pub enum GlobalMsg {
    Quit,
    Esc,
}

#[derive(Debug, Clone)]
pub enum SharedMsg {
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
            ComponentMsg::Shared(s) => ComponentMsg::Shared(s),
        }
    }
}

impl<T> From<T> for ComponentMsg<T> {
    fn from(value: T) -> Self {
        ComponentMsg::Local(value)
    }
}
