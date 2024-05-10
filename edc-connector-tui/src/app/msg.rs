use crate::components::{connectors::msg::ConnectorsMsg, footer::msg::FooterMsg};

#[derive(PartialEq, Debug)]
pub enum AppMsg {
    ConnectorsMsg(ConnectorsMsg),
    ShowFooter,
    FooterMsg(FooterMsg),
}
