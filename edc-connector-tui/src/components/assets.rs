use self::{model::AssetsModel, msg::AssetsMsg};

use super::{
    table::{TableEntry, UiTable},
    Component, ComponentEvent, ComponentMsg, ComponentReturn, SharedMsg,
};
use edc_connector_client::types::{asset::Asset, query::Query};
use futures::FutureExt;
use ratatui::{layout::Rect, widgets::Row, Frame};
pub mod model;
pub mod msg;

#[derive(Default, Debug)]
pub struct Assets;

#[async_trait::async_trait]
impl Component for Assets {
    type Msg = AssetsMsg;

    type Model = AssetsModel;

    fn view(model: &mut Self::Model, f: &mut Frame, area: Rect) {
        UiTable::view(&mut model.table, f, area);
    }

    async fn update(
        model: &mut Self::Model,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        match msg {
            ComponentMsg::Global(_) => Ok(ComponentReturn::empty()),
            ComponentMsg::Shared(SharedMsg::ChangeConnector(c)) => {
                model.client = Some(c.clone());

                return Ok(ComponentReturn::cmd(
                    async move {
                        let assets = c.client().assets().query(Query::default()).await?;
                        Ok(vec![AssetsMsg::FecthedAsset(assets).into()])
                    }
                    .boxed(),
                ));
            }
            ComponentMsg::Local(AssetsMsg::FecthedAsset(assets)) => {
                model.table.elements = assets.into_iter().map(AssetEntry::new).collect();
                Ok(ComponentReturn::empty())
            }
            ComponentMsg::Local(AssetsMsg::TableEvent(table)) => {
                Self::forward_update::<_, UiTable<AssetEntry>>(
                    &mut model.table,
                    ComponentMsg::Local(table),
                    AssetsMsg::TableEvent,
                )
                .await
            }
        }
    }

    fn handle_event(
        model: &Self::Model,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        Self::forward_event::<_, UiTable<AssetEntry>>(&model.table, evt, AssetsMsg::TableEvent)
    }
}

#[derive(Debug)]
pub struct AssetEntry(Asset);

impl AssetEntry {
    pub fn new(asset: Asset) -> AssetEntry {
        AssetEntry(asset)
    }
}

impl TableEntry for AssetEntry {
    fn row(&self) -> Row {
        let properties = serde_json::to_string(self.0.properties()).unwrap();
        let private_properties = serde_json::to_string(self.0.private_properties()).unwrap();
        let data_address = serde_json::to_string(self.0.data_address()).unwrap();
        Row::new(vec![
            self.0.id().to_string(),
            properties,
            private_properties,
            data_address,
        ])
    }

    fn headers() -> Row<'static> {
        Row::new(vec![
            "ID",
            "PROPERTIES",
            "PRIVATE PROPERTIES",
            "DATA ADDRESS",
        ])
    }
}
