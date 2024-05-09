use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Styled},
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Borders, List},
    Frame,
};

use super::{Component, DrawableResource};

#[derive(Debug)]
pub struct ResourceComponent<T> {
    resource: Option<T>,
    name: String,
}

impl<T> Default for ResourceComponent<T> {
    fn default() -> Self {
        Self {
            resource: Default::default(),
            name: String::default(),
        }
    }
}

impl<T: DrawableResource> ResourceComponent<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            resource: None,
        }
    }

    pub fn update_resource(&mut self, resource: Option<T>) {
        self.resource = resource;
    }

    pub fn field_constraints(&self) -> Vec<Constraint> {
        if let Some(res) = self.resource.as_ref() {
            res.fields()
                .into_iter()
                .map(|f| Constraint::Min(1))
                .collect()
        } else {
            vec![]
        }
    }
}

#[async_trait::async_trait]
impl<T: DrawableResource + Send> Component for ResourceComponent<T> {
    type Msg = ();
    type Props = ();

    fn view(&mut self, f: &mut Frame, rect: Rect) {
        let styled_text = Span::styled(
            format!(
                "{}({}) ",
                self.name,
                self.resource.as_ref().map(|a| a.id()).unwrap_or("N/A")
            ),
            Style::default().fg(Color::Red),
        );
        let block = Block::default()
            .title(Title::from(styled_text).alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let constraints = self.field_constraints();

        if !constraints.is_empty() {
            let area = block.inner(rect);
            let layout = Layout::vertical(constraints).split(area);
            if let Some(res) = self.resource.as_ref() {
                let lines = res
                    .fields()
                    .into_iter()
                    .map(|field| {
                        Line::from(vec![
                            format!("{}: ", field.name).set_style(Color::Yellow),
                            field.value.as_ref().to_string().into(),
                        ])
                    })
                    .collect::<Vec<Line>>();

                for (idx, elem) in lines.iter().enumerate() {
                    f.render_widget(elem, layout[idx]);
                }
            }
        }
        // let list = if let Some(res) = self.resource.as_ref() {
        //     let lines = res
        //         .fields()
        //         .into_iter()
        //         .map(|field| {
        //             Line::from(vec![
        //                 format!("{}: ", field.name).set_style(Color::Yellow),
        //                 field.value.as_ref().to_string().into(),
        //             ])
        //         })
        //         .collect::<Vec<Line>>();
        //     List::new(lines).block(block)
        // } else {
        //     List::new(vec!["Test"]).block(block)
        // };

        f.render_widget(block, rect);
    }
}
