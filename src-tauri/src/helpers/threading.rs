use std::sync::mpsc::Receiver;

pub fn receive_flag(channel: &Receiver<bool>, assume: bool) -> bool {
    match channel.try_recv() {
        Ok(result) => return result,
        Err(_) => return assume,
    }
}
