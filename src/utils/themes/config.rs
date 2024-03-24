use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeConfigItem {
    pub r#type: String,
    pub value: String,
    pub name: String,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergedConfigItem {
    pub r#type: String,
    pub display_as: String,
    pub name: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub default: Option<String>,
    pub value: Option<String>,
}
