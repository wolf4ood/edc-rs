use crate::{
    components::{
        assets::AssetsMsg, connectors::msg::ConnectorsMsg, header::msg::HeaderMsg,
        launch_bar::msg::LaunchBarMsg, policies::PoliciesMsg,
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
    HeaderMsg(HeaderMsg),
    RoutingMsg(Nav),
}
