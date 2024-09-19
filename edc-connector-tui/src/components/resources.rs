use std::{fmt::Debug, sync::Arc};

use self::{msg::ResourcesMsg, resource::ResourceComponent};
use super::{
    table::{msg::TableMsg, TableEntry, UiTable},
    Action, Component, ComponentEvent, ComponentMsg, ComponentReturn, Notification,
};
use crate::types::{connector::Connector, info::InfoSheet};
use crossterm::event::{Event, KeyCode};
use futures::future::BoxFuture;
use futures::FutureExt;
use ratatui::{layout::Rect, Frame};
use serde::Serialize;
use std::future::Future;
pub mod msg;
pub mod resource;

pub type ResourceTable<T> = UiTable<T, Box<ResourcesMsg<T>>>;

pub type OnFetch<T> =
    Arc<dyn Fn(&Connector) -> BoxFuture<'static, anyhow::Result<Vec<T>>> + Send + Sync>;

#[derive(Debug)]
pub enum Focus {
    ResourceList,
    Resource,
}

pub struct ResourcesComponent<T: TableEntry> {
    table: ResourceTable<T>,
    resource: ResourceComponent<T>,
    focus: Focus,
    connector: Option<Connector>,
    on_fetch: Option<OnFetch<T>>,
}

impl<T: TableEntry> ResourcesComponent<T> {
    pub fn on_fetch<F, Fut>(mut self, on_fetch: F) -> Self
    where
        F: Fn(Connector) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<Vec<T>>> + Send,
    {
        let handler = Arc::new(on_fetch);
        self.on_fetch = Some(Arc::new(move |conn| {
            let c = conn.clone();
            let inner_handler = handler.clone();
            async move { inner_handler(c).await }.boxed()
        }));

        self
    }

    pub fn info_sheet(&self) -> InfoSheet {
        match self.focus {
            Focus::ResourceList => self.table.info_sheet(),
            Focus::Resource => self.resource.info_sheet(),
        }
    }
}

impl<T: DrawableResource + TableEntry + Clone> Default for ResourcesComponent<T> {
    fn default() -> Self {
        Self {
            table: ResourceTable::new(T::title().to_string())
                .on_select(|res: &T| Box::new(ResourcesMsg::ResourceSelected(res.clone()))),
            resource: ResourceComponent::new(T::title().to_string()),
            focus: Focus::ResourceList,
            connector: None,
            on_fetch: None,
        }
    }
}

#[async_trait::async_trait]
impl<T: DrawableResource + TableEntry + Send + Sync> Component for ResourcesComponent<T> {
    type Msg = ResourcesMsg<T>;
    type Props = Connector;

    async fn init(&mut self, props: Self::Props) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        self.connector = Some(props.clone());

        let connector = props;

        if let Some(on_fetch) = self.on_fetch.as_ref() {
            Ok(ComponentReturn::cmd(
                async move {
                    match on_fetch(&connector).await {
                        Ok(elements) => Ok(vec![ResourcesMsg::ResourcesFetched(elements).into()]),
                        Err(err) => Ok(vec![
                            ResourcesMsg::ResourcesFetchFailed(err.to_string()).into()
                        ]),
                    }
                }
                .boxed(),
            ))
        } else {
            Ok(ComponentReturn::empty())
        }
    }

    fn view(&mut self, f: &mut Frame, rect: Rect) {
        match self.focus {
            Focus::ResourceList => self.table.view(f, rect),
            Focus::Resource => self.resource.view(f, rect),
        }
    }

    async fn update(
        &mut self,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        match msg.to_owned() {
            ResourcesMsg::ResourceSelected(selected) => {
                self.resource.update_resource(Some(selected));
                self.focus = Focus::Resource;
                Ok(ComponentReturn::action(Action::ChangeSheet))
            }
            ResourcesMsg::TableEvent(table) => {
                Self::forward_update(&mut self.table, table.into(), ResourcesMsg::TableEvent).await
            }
            ResourcesMsg::ResourcesFetched(resources) => {
                self.table.elements = resources;
                Ok(ComponentReturn::empty())
            }
            ResourcesMsg::Back => {
                self.focus = Focus::ResourceList;
                Ok(ComponentReturn::action(Action::ChangeSheet))
            }
            ResourcesMsg::ResourceMsg(msg) => {
                Self::forward_update(&mut self.resource, msg.into(), ResourcesMsg::ResourceMsg)
                    .await
            }
            ResourcesMsg::ResourcesFetchFailed(error) => Ok(ComponentReturn::action(
                Action::Notification(Notification::error(error)),
            )),
        }
    }

    fn handle_event(
        &mut self,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        match self.focus {
            Focus::ResourceList => Self::forward_event(&mut self.table, evt, |msg| match msg {
                TableMsg::Local(table) => ResourcesMsg::TableEvent(TableMsg::Local(table)),
                TableMsg::Outer(outer) => *outer,
            }),
            Focus::Resource => match evt {
                ComponentEvent::Event(Event::Key(k)) if k.code == KeyCode::Esc => {
                    Ok(vec![ResourcesMsg::Back.into()])
                }
                _ => Self::forward_event(&mut self.resource, evt, ResourcesMsg::ResourceMsg),
            },
        }
    }
}

pub trait DrawableResource {
    fn id(&self) -> &str;

    fn title() -> &'static str;

    fn fields(&self) -> Vec<Field>;
}

pub struct Field {
    name: String,
    value: FieldValue,
}

impl Field {
    pub fn new(name: String, value: FieldValue) -> Self {
        Self { name, value }
    }

    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: FieldValue::Str(value.into()),
        }
    }

    pub fn json<T: Serialize + ?Sized>(name: impl Into<String>, value: &T) -> Self {
        Self {
            name: name.into(),
            value: FieldValue::Json(serde_json::to_string_pretty(value).unwrap()),
        }
    }
}

pub enum FieldValue {
    Str(String),
    Json(String),
}

impl AsRef<str> for FieldValue {
    fn as_ref(&self) -> &str {
        match self {
            FieldValue::Str(s) => s,
            FieldValue::Json(s) => s,
        }
    }
}
