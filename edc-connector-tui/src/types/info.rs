#[derive(Default)]
pub struct Sheet(Vec<(String, String)>);

impl Sheet {
    pub fn add(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.0.push((key.into(), value.into()));
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.0.iter()
    }
}
