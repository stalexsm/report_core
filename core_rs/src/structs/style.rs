use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Style {
    val: String,
}

impl Style {
    pub fn new(val: &str) -> Self {
        Style {
            val: val.to_string(),
        }
    }
}
