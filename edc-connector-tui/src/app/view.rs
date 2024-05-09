use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::constants::{BANNER, HIGHLIGHT_COLOR};

use super::App;

impl App {
    pub fn view(&self, f: &mut Frame) {
        let main = self.main_layout(f);

        f.render_widget(self.header(), main[0]);
        f.render_widget(self.body(), main[1]);
        f.render_widget(self.footer(), main[2]);
    }

    fn header(&self) -> Paragraph {
        let top_text = Text::from(BANNER).patch_style(Style::default().fg(HIGHLIGHT_COLOR));
        Paragraph::new(top_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Right)
            .block(Block::default())
    }

    fn body(&self) -> Block {
        let styled_text = Span::styled(
            "Connectors",
            Style::default().fg(Color::Red).bg(Color::Yellow),
        );
        Block::default()
            .title(Title::from(styled_text).alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
    }

    fn footer(&self) -> Block {
        Block::default()
    }

    fn main_layout(&self, f: &mut Frame) -> Rc<[Rect]> {
        let window = f.size();
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(85),
                    Constraint::Percentage(5),
                ]
                .as_ref(),
            )
            .split(window)
    }
}
