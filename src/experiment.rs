use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rocket::serde::Deserialize;
use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};

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
    device: SerialPortBuilder,
    pump: Option<String>,
}

pub type RunState = Arc<AtomicBool>;

impl Driver {
    pub fn new(dev: &str) -> Self {
        Self {
            server_url: String::from("http://localhost:8000"),
            device: serialport::new(dev, 9_600)
                .data_bits(DataBits::Eight)
                .parity(Parity::None)
                .stop_bits(StopBits::One)
                .flow_control(FlowControl::Software)
                .timeout(Duration::MAX),
            pump: None,
        }
    }

    pub fn run_experiment(&self, experiment: &ExperimentInfo, running: RunState) {
        let port = &self.device.clone().open().unwrap();
        port.clear(ClearBuffer::All).unwrap();

        while running.load(Ordering::SeqCst) {
            println!(
                "{}, {}",
                experiment.experiment_name, experiment.experiment_id
            );
            thread::sleep(Duration::from_millis(2000));
        }
    }
}
