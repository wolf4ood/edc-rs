use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Styled},
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
    Frame,
};

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
                for (idx, elem) in res.fields().into_iter().enumerate() {
                    let mut field = FieldComponent::new(elem);
                    field.view(f, layout[idx]);
                }
            }
        }

        f.render_widget(block, rect);
    }
}

pub struct FieldComponent(Field);

impl FieldComponent {
    pub fn new(field: Field) -> Self {
        Self(field)
    }
}

impl Component for FieldComponent {
    type Msg = ();

    type Props = Field;

    fn view(&mut self, f: &mut Frame, rect: Rect) {
        let layout = Layout::horizontal(vec![Constraint::Min(2), Constraint::Min(2)]).split(rect);
        let line = Line::from(vec![format!("{}: ", self.0.name).set_style(Color::Yellow)]);
        let name = Paragraph::new(line);
        let value = Paragraph::new(self.0.value.as_ref()).block(Block::bordered());

        f.render_widget(name, layout[0]);
        f.render_widget(value, layout[1]);
    }
}
