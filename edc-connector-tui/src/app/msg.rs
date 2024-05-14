use crate::components::{
    assets::msg::AssetsMsg, connectors::msg::ConnectorsMsg, footer::msg::FooterMsg,
};

#[derive(Debug)]
pub enum AppMsg {
    ConnectorsMsg(ConnectorsMsg),
    ShowFooter,
    FooterMsg(FooterMsg),
    AssetsMsg(AssetsMsg),
}
