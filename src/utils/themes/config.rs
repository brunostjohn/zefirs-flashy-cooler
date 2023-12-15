use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeConfigItem {
    pub r#type: String,
    pub value: String,
    pub name: String,
}
