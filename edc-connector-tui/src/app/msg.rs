use crate::components::{
    assets::msg::AssetsMsg, connectors::msg::ConnectorsMsg, footer::msg::FooterMsg,
};

#[derive(PartialEq, Debug)]
pub enum AppMsg {
    ConnectorsMsg(ConnectorsMsg),
    ShowFooter,
    FooterMsg(FooterMsg),
    AssetsMsg(AssetsMsg),
}
