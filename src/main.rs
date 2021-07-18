#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::experiment::{ExperimentInfo, RunState};
use crate::state::TState;

mod experiment;
mod state;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/run_experiment", format = "json", data = "<message>")]
fn run_experiment_handler(
    message: Json<ExperimentInfo>,
    run_state: &State<RunState>,
    server_state: &State<Mutex<TState>>,
) -> Status {
    let running = run_state.load(Ordering::SeqCst);

    if running {
        return Status::ServiceUnavailable;
    } else {
        let driver_clone = server_state.lock().unwrap().driver.clone();
        let worker_status_clone = Arc::clone(run_state);
        server_state.lock().unwrap().threads.push(thread::spawn({
            move || driver_clone.run_experiment(&message, worker_status_clone)
        }));
        run_state.swap(true, Ordering::SeqCst);
    }

    Status::Ok
}

#[get("/stop_experiment")]
fn stop_experiment_handler(
    run_state: &State<RunState>,
    server_state: &State<Mutex<TState>>,
) -> Status {
    let running = run_state.load(Ordering::SeqCst);

    if !running {
        Status::ServiceUnavailable
    } else {
        run_state.swap(false, Ordering::SeqCst);
        let t = server_state.lock().unwrap().threads.pop().unwrap();

        return match t.join() {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        };
    }
}

#[catch(404)]
fn not_found() -> Status {
    Status::NotFound
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, run_experiment_handler, stop_experiment_handler],
        )
        .register("/", catchers![not_found])
        .manage(Mutex::new(TState::new()))
        .manage(Arc::new(AtomicBool::new(false)))
}
