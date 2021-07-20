#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

use crate::state::TState;
use crate::task::{RunState, TaskInfo};

mod state;
mod task;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/run_task", format = "json", data = "<message>")]
fn run_task_handler(
    message: Json<TaskInfo>,
    run_state: &State<RunState>,
    server_state: &State<Mutex<TState>>,
) -> Custom<Option<String>> {
    let running = run_state.load(Ordering::SeqCst);

    if running {
        status::Custom(
            Status::ServiceUnavailable,
            Some(String::from("Service is already running")),
        )
    } else {
        let worker_status_clone = Arc::clone(run_state);
        let driver_clone = server_state.lock().unwrap().driver.clone();
        let port_check = driver_clone.check_port();
        return match port_check {
            Ok(msg) => {
                server_state.lock().unwrap().threads.push(thread::spawn({
                    move || driver_clone.run_task(&message, worker_status_clone)
                }));
                run_state.swap(true, Ordering::SeqCst);
                status::Custom(Status::Accepted, Some(msg))
            }
            Err(msg) => status::Custom(Status::InternalServerError, Some(msg)),
        };
    }
}

#[get("/stop_task")]
fn stop_task_handler(run_state: &State<RunState>, server_state: &State<Mutex<TState>>) -> Status {
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
        .mount("/", routes![index, run_task_handler, stop_task_handler])
        .register("/", catchers![not_found])
        .manage(Mutex::new(TState::new()))
        .manage(Arc::new(AtomicBool::new(false)))
}
