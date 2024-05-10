use tui_textarea::Input;

#[derive(Debug, PartialEq)]
pub enum FooterMsg {
    AppendCommand(Input),
}
