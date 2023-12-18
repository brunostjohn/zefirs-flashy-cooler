use super::*;
use crate::ULPlatformBuilder;
use rusty_fork::rusty_fork_test;

rusty_fork_test! {
    #[test]
    fn creates_renderer() {
        ULPlatformBuilder::new()
            .enable_file_logger("./logs.txt")
            .enable_platform_file_system()
            .enable_platform_font_loader()
            .build();

        ULRendererBuilder::new()
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
    }
}
