use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub r#type: String,
    pub display_as: String,
    pub name: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub default: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub fs_name: Option<String>,
    pub tested_on: Option<Vec<String>>,
    pub customisable_parameters: Vec<Parameter>,
}
