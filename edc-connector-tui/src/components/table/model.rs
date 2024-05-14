use ratatui::widgets::TableState;

#[derive(Debug)]
pub struct TableModel<T> {
    pub elements: Vec<T>,
    pub table_state: TableState,
    pub name: String,
}

impl<T> TableModel<T> {
    pub fn new(name: String) -> Self {
        Self {
            elements: vec![],
            table_state: TableState::default().with_selected(0),
            name,
        }
    }
}

impl<T> Default for TableModel<T> {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            table_state: Default::default(),
            name: Default::default(),
        }
    }
}
