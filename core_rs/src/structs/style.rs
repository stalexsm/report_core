use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Style {
    style_id: String,
}

impl Style {
    pub fn new(val: &str) -> Self {
        Style {
            style_id: val.to_string(),
        }
    }

    pub fn get_id(&self) -> String {
        self.style_id.to_string()
    }
}
