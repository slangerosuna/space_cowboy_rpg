use pyo3::prelude::*;
use bevy::prelude::*;

use std::sync::{Arc, Mutex};

use crate::Config;

#[pyclass]
#[derive(Clone)]
pub struct App {
    config: Config,

    handle_request_queue: Vec<Arc<Mutex<FnRequest<(usize, String), String>>>>,
    new_request_thread_queue: Vec<Arc<Mutex<FnRequest<String, usize>>>>,
}

struct FnRequest<Arg, Ret> {
    arg: Arg,
    ret: Option<Ret>,
}

#[pymethods]
impl App {
    #[new]
    fn new(config: Config) -> Self {
        Self {
            config,

            handle_request_queue: Vec::new(),
            new_request_thread_queue: Vec::new(),
        }
    }

    fn init(&self) -> PyResult<()> {
        Ok(())
    }

    fn check_requests(&self) -> Option<(usize, String)> {
        if self.handle_request_queue.is_empty() {
            None
        } else {
            let request = self.handle_request_queue[0].lock().unwrap().arg.clone();
            Some(request)
        }
    }

    fn handle_request(&mut self, response: String) {
        let request = self.handle_request_queue.remove(0);
        request.lock().unwrap().ret = Some(response);
    }

    fn check_new_thread_requests(&self) -> Option<String> {
        if self.new_request_thread_queue.is_empty() {
            None
        } else {
            let request = self.new_request_thread_queue[0].lock().unwrap().arg.clone();
            Some(request)
        }
    }

    fn new_thread(&mut self, id: usize) {
        let request = self.new_request_thread_queue.remove(0);
        request.lock().unwrap().ret = Some(id);
    }
}