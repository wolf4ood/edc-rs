use crate::{
    components::{
        assets::AssetsMsg, connectors::msg::ConnectorsMsg,
        contract_definitions::ContractDefinitionsMsg, header::msg::HeaderMsg,
        launch_bar::msg::LaunchBarMsg, policies::PoliciesMsg, NotificationMsg,
    },
    types::nav::Nav,
};

#[derive(Debug)]
pub enum AppMsg {
    ConnectorsMsg(ConnectorsMsg),
    ShowLaunchBar,
    HideLaunchBar,
    LaunchBarMsg(LaunchBarMsg),
    AssetsMsg(AssetsMsg),
    PoliciesMsg(PoliciesMsg),
    ContractDefinitions(ContractDefinitionsMsg),
    HeaderMsg(HeaderMsg),
    RoutingMsg(Nav),
    NontificationMsg(NotificationMsg),
    ChangeSheet,
}
