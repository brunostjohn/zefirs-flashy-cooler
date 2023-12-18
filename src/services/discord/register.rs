use anyhow::Context;
use discord_sdk::{
    handlers::Printer,
    registration::{Application, BinArg, LaunchCommand},
    Discord, Subscriptions,
};

const APP_ID: i64 = 1185981801747988620;

pub fn register() -> anyhow::Result<Discord> {
    let exe_location = LaunchCommand::Bin {
        path: std::env::current_exe().context("Failed to find executable location!")?,
        args: vec![BinArg::Arg("--ignore-minimised".into())],
    };

    let app = Application {
        id: APP_ID,
        name: Some("Zefir's Flashy Cooler".into()),
        command: exe_location,
    };

    discord_sdk::registration::register_app(app).context("Failed to register app with Discord!")?;

    // the developer seriously didnt derive clone?? silly
    let exe_location = LaunchCommand::Bin {
        path: std::env::current_exe().context("Failed to find executable location!")?,
        args: vec![BinArg::Arg("--ignore-minimised".into())],
    };

    let app = Application {
        id: APP_ID,
        name: Some("Zefir's Flashy Cooler".into()),
        command: exe_location,
    };

    let discord = Discord::new(app, Subscriptions::ACTIVITY, Box::new(Printer))
        .context("Failed to create Discord client!")?;

    Ok(discord)
}
