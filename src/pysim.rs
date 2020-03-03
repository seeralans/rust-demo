use pyo3::prelude::*;
use pyo3::{wrap_pymodule, wrap_pyfunction};
use ndarray;
use ndarray::{Array};
use numpy::{IntoPyArray, PyArray1};

use super::simulator;

#[pymodule(pysim)]
pub fn simulator_py(_py: Python, module: &PyModule) -> PyResult<()> {
  module.add_wrapped(wrap_pyfunction!(coverage_sim))?;
  Ok(())
}

#[pyfunction]
/// coverage(n0, N, p, num_trials)
/// --
/// blah blah
fn coverage_sim(py: Python, n0: i64, p: f64, N: i64, num_trials: i64) -> Py<PyArray1<u64>>{
  let data = simulator::coverage_sim(n0, p, N as usize, num_trials as usize);
  data.into_pyarray(py).to_owned()
}
