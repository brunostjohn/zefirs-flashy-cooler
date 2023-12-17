use anyhow::Context;
use discord_sdk::{
    activity::{ActivityBuilder, Assets, Button},
    Discord,
};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn activity_handler(
    set_to: String,
    discord: State<'_, Arc<RwLock<Option<Discord>>>>,
) -> Result<(), String> {
    match set_to.as_str() {
        "browsing_themes" => {
            set_to_browsing_themes(&discord)
                .await
                .map_err(|e| e.to_string())?;
        }
        "clear" => {
            clear_activity(&discord).await.map_err(|e| e.to_string())?;
        }
        _ => {
            return Err(format!("Unknown activity: {}", set_to));
        }
    }

    Ok(())
}

pub async fn set_to_browsing_themes(discord: &Arc<RwLock<Option<Discord>>>) -> anyhow::Result<()> {
    if let Some(discord) = discord.read().await.as_ref() {
        let activity = ActivityBuilder::new()
            .state("Browsing themes.")
            .details("Currently checking out themes.")
            .button(Button {
                label: "Check it out yourself!".into(),
                url: "https://zefirsflashycooler.app".into(),
            })
            .assets(Assets {
                large_image: Some("default-img".into()),
                large_text: Some("Zefir's Flashy Cooler".into()),
                ..Default::default()
            });

        discord
            .update_activity(activity)
            .await
            .context("Failed to update activity!")?;
    }

    Ok(())
}

pub async fn clear_activity(discord: &Arc<RwLock<Option<Discord>>>) -> anyhow::Result<()> {
    if let Some(discord) = discord.read().await.as_ref() {
        discord
            .clear_activity()
            .await
            .context("Failed to clear activity!")?;
    }

    Ok(())
}
