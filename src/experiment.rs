use rocket::serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct ExperimentInfo {
    experiment_name: String,
    experiment_id: u32,
    emulsifier_volume: u32,
    measurement_interval: u32,
}

#[derive(Clone, Debug)]
pub struct Driver {
    server_url: String,
    device: String,
    pump: Option<String>,
}

pub type RunState = Arc<AtomicBool>;

impl Driver {
    pub fn new() -> Self {
        Self {
            server_url: String::from("http://localhost:8000"),
            device: String::from("/dev/ttyUSB0"),
            pump: None,
        }
    }

    pub fn run_experiment(&self, experiment: &ExperimentInfo, running: RunState) {
        while running.load(Ordering::SeqCst) {
            println!(
                "{}, {}",
                experiment.experiment_name, experiment.experiment_id
            );
            thread::sleep(Duration::from_millis(2000));
        }
    }
}
