use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdResponse<T> {
    #[serde(rename = "@id")]
    id: T,
    created_at: i64,
}

impl<T> IdResponse<T> {
    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }
}
