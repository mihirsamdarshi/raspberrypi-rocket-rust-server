use std::thread;

use crate::task::Driver;

pub struct TState {
    pub driver: Driver,
    pub threads: Vec<thread::JoinHandle<()>>,
}

impl TState {
    pub fn new() -> Self {
        Self {
            driver: Driver::new("/dev/ttyUSB0"),
            threads: vec![],
        }
    }
}
