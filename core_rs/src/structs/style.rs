use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Style {
    #[serde(rename = "style_id")]
    id: String,
}

impl Style {
    pub fn new(val: &str) -> Self {
        Style {
            id: val.to_string(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}
