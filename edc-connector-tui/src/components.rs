use std::fmt::Debug;

use crossterm::event::Event;
use futures::{future::BoxFuture, FutureExt};
use ratatui::{layout::Rect, Frame};

use crate::{nav::Nav, types::connector::Connector};

pub mod assets;
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
        F: Fn(C::Msg) -> Self::Msg + Send + Sync,
        F: 'static,
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

#[derive(Default)]
pub struct ComponentReturn<T> {
    pub(crate) msgs: Vec<ComponentMsg<T>>,
    pub(crate) cmds: Vec<BoxFuture<'static, anyhow::Result<Vec<ComponentMsg<T>>>>>,
}

impl<T: Debug> Debug for ComponentReturn<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentReturn")
            .field("msgs", &self.msgs)
            .field("cmds", &self.cmds.len())
            .finish()
    }
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
            msgs: vec![],
            cmds: vec![],
        }
    }

    pub fn map<M, F>(self, mapper: F) -> ComponentReturn<M>
    where
        F: Fn(T) -> M + Sync + Send,
        F: 'static,
    {
        let msgs = self
            .msgs
            .into_iter()
            .map(|msg| Self::map_msg(msg, &mapper))
            .collect();

        let cmds = self
            .cmds
            .into_iter()
            .map(|fut| {
                let inner_mapper = mapper;
                async move {
                    let msgs = fut.await?;

                    Ok(msgs
                        .into_iter()
                        .map(|msg| Self::map_msg(msg, &inner_mapper))
                        .collect())
                }
                .boxed()
            })
            .collect::<Vec<_>>();

        ComponentReturn { msgs, cmds }
    }

    pub fn merge(mut self, mut other: ComponentReturn<T>) -> ComponentReturn<T> {
        self.cmds.append(&mut other.cmds);
        self.msgs.append(&mut other.msgs);
        self
    }

    fn map_msg<M, F>(msg: ComponentMsg<T>, mapper: &F) -> ComponentMsg<M>
    where
        F: Fn(T) -> M + Sync + Send,
    {
        match msg {
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

impl<T> From<ComponentMsg<T>> for ComponentReturn<T> {
    fn from(value: ComponentMsg<T>) -> Self {
        ComponentReturn {
            msgs: vec![value],
            cmds: vec![],
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
