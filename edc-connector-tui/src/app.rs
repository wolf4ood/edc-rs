use std::rc::Rc;
mod action;
mod fetch;
pub mod model;
mod msg;

use crossterm::event::{self, Event, KeyCode};
use edc_connector_client::EdcConnectorClient;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    components::{
        assets::{AssetEntry, AssetsComponent},
        connectors::Connectors,
        footer::Footer,
        header::HeaderComponent,
        launch_bar::LaunchBar,
        policies::{PolicyDefinitionEntry, PolicyDefinitionsComponent},
        resources::ResourcesComponent,
        Component, ComponentEvent, ComponentMsg, ComponentReturn,
    },
    config::Config,
    types::{
        connector::Connector,
        nav::{Menu, Nav},
    },
};

use self::{model::AppFocus, msg::AppMsg};

pub struct App {
    connectors: Connectors,
    policies: PolicyDefinitionsComponent,
    assets: AssetsComponent,
    launch_bar: LaunchBar,
    launch_bar_visible: bool,
    focus: AppFocus,
    header: HeaderComponent,
    footer: Footer,
}

impl App {
    pub fn init(cfg: Config) -> App {
        let connectors = cfg
            .connectors
            .into_iter()
            .map(|cfg| {
                let client = EdcConnectorClient::builder()
                    .management_url(cfg.address())
                    .build()
                    .unwrap();
                Connector::new(cfg, client)
            })
            .collect();
        let connectors = Connectors::new(connectors);

        App {
            connectors,
            policies: PolicyDefinitionsComponent::default().on_fetch(Self::fetch_policies),
            assets: AssetsComponent::default().on_fetch(Self::fetch_assets),
            launch_bar: LaunchBar::default(),
            launch_bar_visible: false,
            focus: AppFocus::ConnectorList,
            footer: Footer::default(),
            header: HeaderComponent::default(),
        }
    }

    pub async fn handle_routing(&mut self, nav: Nav) -> anyhow::Result<ComponentReturn<AppMsg>> {
        self.launch_bar_visible = false;
        self.launch_bar.clear();
        self.header.set_selected_menu(nav);

        match self.header.selected_menu() {
            Menu::Connectors => {
                self.focus = AppFocus::ConnectorList;
            }
            Menu::Assets => {
                self.focus = AppFocus::Assets;
                if let Some(connector) = self.connectors.selected() {
                    return Self::forward_init(
                        &mut self.assets,
                        connector.clone(),
                        AppMsg::AssetsMsg,
                    )
                    .await;
                }
            }
            Menu::Policies => {
                self.focus = AppFocus::Policies;
                if let Some(connector) = self.connectors.selected() {
                    return Self::forward_init(
                        &mut self.policies,
                        connector.clone(),
                        AppMsg::PoliciesMsg,
                    )
                    .await;
                }
            }
            Menu::ContractDefinitions => {}
        };

        Ok(ComponentReturn::empty())
    }
}

#[async_trait::async_trait]
impl Component for App {
    type Msg = AppMsg;
    type Props = ();

    fn view(&mut self, f: &mut Frame, rect: Rect) {
        let main = self.main_layout(rect);

        self.header.view(f, main[0]);
        self.launch_bar.view(f, main[1]);

        match self.header.selected_menu() {
            Menu::Connectors => self.connectors.view(f, main[2]),
            Menu::Assets => self.assets.view(f, main[2]),
            Menu::Policies => self.policies.view(f, main[2]),
            Menu::ContractDefinitions => self.policies.view(f, main[2]),
        }

        self.footer.view(f, main[3]);
    }

    async fn update(
        &mut self,
        msg: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<AppMsg>> {
        match msg.to_owned() {
            AppMsg::ConnectorsMsg(m) => {
                Self::forward_update::<_, Connectors>(
                    &mut self.connectors,
                    m.into(),
                    AppMsg::ConnectorsMsg,
                )
                .await
            }
            AppMsg::ShowLaunchBar => {
                self.launch_bar_visible = true;
                self.focus = AppFocus::LaunchBar;
                Ok(ComponentReturn::empty())
            }
            AppMsg::HideLaunchBar => {
                self.launch_bar.clear();
                self.launch_bar_visible = false;
                self.focus = AppFocus::ConnectorList;
                Ok(ComponentReturn::empty())
            }
            AppMsg::LaunchBarMsg(m) => {
                Self::forward_update::<_, LaunchBar>(
                    &mut self.launch_bar,
                    m.into(),
                    AppMsg::LaunchBarMsg,
                )
                .await
            }
            AppMsg::AssetsMsg(m) => {
                Self::forward_update::<_, ResourcesComponent<AssetEntry>>(
                    &mut self.assets,
                    m.into(),
                    AppMsg::AssetsMsg,
                )
                .await
            }
            AppMsg::PoliciesMsg(m) => {
                Self::forward_update::<_, ResourcesComponent<PolicyDefinitionEntry>>(
                    &mut self.policies,
                    m.into(),
                    AppMsg::PoliciesMsg,
                )
                .await
            }
            AppMsg::HeaderMsg(m) => {
                Self::forward_update(&mut self.header, m.into(), AppMsg::HeaderMsg).await
            }
            AppMsg::RoutingMsg(nav) => self.handle_routing(nav).await,
        }
    }

    fn handle_event(
        &mut self,
        evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        let msg = match self.focus {
            AppFocus::ConnectorList => Self::forward_event::<_, Connectors>(
                &mut self.connectors,
                evt.clone(),
                AppMsg::ConnectorsMsg,
            )?,
            AppFocus::LaunchBar => Self::forward_event::<_, LaunchBar>(
                &mut self.launch_bar,
                evt.clone(),
                AppMsg::LaunchBarMsg,
            )?,
            AppFocus::Assets => {
                Self::forward_event(&mut self.assets, evt.clone(), AppMsg::AssetsMsg)?
            }
            AppFocus::Policies => Self::forward_event::<
                _,
                ResourcesComponent<PolicyDefinitionEntry>,
            >(
                &mut self.policies, evt.clone(), AppMsg::PoliciesMsg
            )?,
        };

        let msg = if msg.is_empty() {
            Self::forward_event(&mut self.header, evt.clone(), AppMsg::HeaderMsg)?
        } else {
            msg
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
    fn main_layout(&self, rect: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Percentage(if self.launch_bar_visible { 5 } else {0}),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(rect)
    }

    fn handle_key(key: event::KeyEvent) -> Vec<ComponentMsg<AppMsg>> {
        match key.code {
            KeyCode::Char(':') => vec![(AppMsg::ShowLaunchBar.into())],
            _ => vec![],
        }
    }
}
