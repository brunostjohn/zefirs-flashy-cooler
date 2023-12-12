pub trait DisplayCooler {
    async fn initialise(&mut self) -> anyhow::Result<()>;
    async fn close(&mut self) -> anyhow::Result<()>;
    async fn reopen(&mut self) -> anyhow::Result<()>;
    async fn send_image(&mut self, image: &[u8]) -> anyhow::Result<()>;
}
