use std::thread;

use crate::experiment::Driver;

pub struct TState {
    pub driver: Driver,
    pub threads: Vec<thread::JoinHandle<()>>,
}

impl TState {
    pub fn new() -> Self {
        Self {
            driver: Driver::new(),
            threads: vec![],
        }
    }
}
