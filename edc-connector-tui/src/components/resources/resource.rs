use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::components::StatelessComponent;

use super::{Component, DrawableResource, Field, FieldValue};

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
                .map(|f| match f.value {
                    FieldValue::Str(_) => Constraint::Length(3),
                    FieldValue::Json(_) => Constraint::Min(5),
                })
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
                " {}({}) ",
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
                for (idx, elem) in res.fields().into_iter().enumerate() {
                    let mut field = FieldComponent;
                    field.view(&elem, f, layout[idx]);
                }
            }
        }

        f.render_widget(block, rect);
    }
}

pub struct FieldComponent;

impl StatelessComponent for FieldComponent {
    type Props = Field;

    fn view(&mut self, props: &Self::Props, f: &mut Frame, rect: Rect) {
        let styled_text = Span::styled(
            format!(" {} ", props.name),
            Style::default().fg(Color::Yellow),
        );
        let value =
            Paragraph::new(props.value.as_ref()).block(Block::bordered().title(styled_text));

        f.render_widget(value, rect)
    }
}
