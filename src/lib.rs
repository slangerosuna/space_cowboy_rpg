use pyo3::prelude::*;

use serde::{Deserialize, Serialize};
use toml;

mod app;

use app::App;

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub worker_threads: usize,
}

#[pyfunction]
fn load_config(path: String) -> PyResult<Config> {
    let contents = std::fs::read_to_string(path).unwrap();
    toml::from_str(&contents).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pymethods]
impl Config {
    #[new]
    fn new(worker_threads: usize) -> Self {
        Config {
            worker_threads,
        }
    }

    fn save(&self, path: String) -> PyResult<()> {
        let contents = toml::to_string(self).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        std::fs::write(path, contents).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(())
    }
}

#[pymodule]
fn rpg_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_config, m)?)?;
    m.add_class::<Config>()?;
    m.add_class::<App>()?;
    Ok(())
}