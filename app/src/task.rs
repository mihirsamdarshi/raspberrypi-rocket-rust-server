use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rocket::serde::Deserialize;
use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};

#[derive(Deserialize, Debug)]
pub struct TaskInfo {
    task_name: String,
    task_id: u32,
    additive_volume: u32,
    measurement_interval: u32,
}

#[derive(Clone, Debug)]
pub struct Driver {
    device: SerialPortBuilder,
}

pub type RunState = Arc<AtomicBool>;

impl Driver {
    pub fn new(dev: &str) -> Self {
        Self {
            device: serialport::new(dev, 9_600)
                .data_bits(DataBits::Eight)
                .parity(Parity::None)
                .stop_bits(StopBits::One)
                .flow_control(FlowControl::Software)
                .timeout(Duration::MAX),
        }
    }

    pub fn check_port(&self) -> Result<String, String> {
        let port_res = &self.device.clone().open();
        match port_res {
            Ok(p) => Ok(format!(
                "Port check initialized successfully {}",
                p.name().unwrap(),
            )),
            Err(e) => Err(format!(
                "Port check unable to initialize successfully. Error: {}",
                e.description
            )),
        }
    }

    pub fn run_task(&self, task: &TaskInfo, running: RunState) {
        let port_res = &self.device.clone().open();
        let port = port_res.as_ref().expect("Unable to initialize port");
        port.clear(ClearBuffer::All).unwrap();

        while running.load(Ordering::SeqCst) {
            println!("{}, {}", task.task_name, task.task_id);
            thread::sleep(Duration::from_millis(2000));
        }
    }
}
