use crate::{ULPlatformBuilder, ULRendererBuilder};
use rusty_fork::rusty_fork_test;

use super::*;

rusty_fork_test! {
    #[test]
    fn creates_view() {
        ULPlatformBuilder::new()
            .enable_file_logger("./logs.txt")
            .enable_platform_file_system()
            .enable_platform_font_loader()
            .build();
        let renderer = ULRendererBuilder::new()
            .set_resource_path_prefix({
                let mut newthing = std::env::current_dir().unwrap();
                newthing.pop();
                newthing.pop();
                newthing.push("target");
                newthing.push("debug");
                newthing.push("deps");
                newthing.push("resources\\");

                newthing
            })
            .build();
        let _ = ULViewBuilder::new(&renderer)
            .set_width(480)
            .set_height(480)
            .build();
    }

    #[test]
    fn loads_with_future() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                ULPlatformBuilder::new()
                    .enable_file_logger("./logs.txt")
                    .enable_platform_file_system()
                    .enable_platform_font_loader()
                    .build();
                let renderer = ULRendererBuilder::new()
                    .set_resource_path_prefix({
                        let mut newthing = std::env::current_dir().unwrap();
                        newthing.pop();
                        newthing.pop();
                        newthing.push("target");
                        newthing.push("debug");
                        newthing.push("deps");
                        newthing.push("resources\\");

                        newthing
                    })
                    .build();
                let mut view = ULViewBuilder::new(&renderer)
                    .set_width(480)
                    .set_height(480)
                    .build();

                view.load_url("https://www.google.com")
                    .await
                    .expect("Failed to load URL");
            })
    }
}
