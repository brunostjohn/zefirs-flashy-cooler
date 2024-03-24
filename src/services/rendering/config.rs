use crate::{
    services::sensors::SensorMessage,
    utils::themes::{config::ThemeConfigItem, paths::get_all_themes_path},
};
use anyhow::Context;
use tachyonix::Sender;
use tokio::fs;
use ultralight::ULView;

pub async fn load_theme_with_config(
    view: &mut ULView<'_>,
    fs_name: &str,
    sensor_sender: &Sender<SensorMessage>,
) -> anyhow::Result<()> {
    view.reload();

    // load non-sensor values
    let theme_config_path = get_all_themes_path().join(fs_name).join("config.json");
    let theme_config_unparsed = fs::read_to_string(theme_config_path)
        .await
        .context("Failed to read theme config file!")?;
    let theme_config = serde_json::from_str::<Vec<ThemeConfigItem>>(&theme_config_unparsed)
        .context("Failed to parse theme config file!")?;
    let mut event_string = String::from("{");
    theme_config
        .iter()
        .filter(|item| item.r#type != "sensor")
        .flat_map(serde_json::to_string)
        .zip(theme_config.iter().filter(|item| item.r#type != "sensor"))
        .for_each(|(serialised, item)| {
            event_string.push('\"');
            event_string.push_str(&item.name);
            event_string.push_str("\":");
            event_string.push_str(&serialised);
            event_string.push(',');
        });
    event_string.pop();
    event_string.push('}');

    let script = format!(
        "document.dispatchEvent(new CustomEvent('configLoaded', {{ detail: JSON.parse('{}') }}));",
        event_string
    );

    view.evaluate_script(script);

    // load sensor values
    let sensor_values = theme_config
        .iter()
        .filter(|item| item.r#type == "sensor" && item.path.is_some())
        .map(
            |ThemeConfigItem {
                 value, name, path, ..
             }| (value, name, path.as_ref().unwrap()),
        )
        .collect::<Vec<_>>();

    Ok(())
}

async fn subscribe_to_sensors(
    values: &Vec<(&String, &String, &String)>,
    sensor_sender: &Sender<SensorMessage>,
) {
    //
}
