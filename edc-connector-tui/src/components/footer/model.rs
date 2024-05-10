use tui_textarea::TextArea;

#[derive(Default, Debug)]
pub struct FooterModel {
    pub(crate) area: TextArea<'static>,
}
