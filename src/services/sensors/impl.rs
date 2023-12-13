use tokio::time::{sleep, Duration};

pub struct Sensors {}

impl Sensors {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&mut self) {
        loop {
            println!("Sensors");
            sleep(Duration::from_secs(1)).await;
        }
    }
}
