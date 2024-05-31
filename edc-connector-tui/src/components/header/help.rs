use ratatui::{
    layout::Rect,
    style::{Color, Style, Styled},
    text::{Line, Span},
    widgets::{Block, List},
    Frame,
};

use crate::{components::StatelessComponent, types::info::InfoSheet};

#[derive(Default)]
pub struct InfoComponent {}

#[async_trait::async_trait]
impl StatelessComponent for InfoComponent {
    type Props = InfoSheet;

    fn view(&mut self, props: &Self::Props, f: &mut Frame, rect: Rect) {
        let list = props
            .iter_info()
            .map(|(name, value)| {
                Line::from(vec![
                    name.to_string()
                        .set_style(Style::default().fg(Color::Yellow)),
                    Span::raw(":").set_style(Style::default().fg(Color::Yellow)),
                    Span::raw(value),
                ])
            })
            .collect::<List>()
            .block(Block::default());

        f.render_widget(list, rect)
    }
}
