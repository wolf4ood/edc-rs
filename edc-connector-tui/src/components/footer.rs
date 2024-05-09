use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::Component;

pub mod msg;

#[derive(Default)]
pub struct Footer {}

#[async_trait::async_trait]
impl Component for Footer {
    type Msg = ();
    type Props = ();

    fn view(&mut self, f: &mut Frame, rect: Rect) {
        let block = Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded);
        f.render_widget(block, rect)
    }
}
