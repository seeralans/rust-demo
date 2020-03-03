use pyo3::prelude::*;
use pyo3::{wrap_pymodule, wrap_pyfunction};


use ndarray;
use ndarray::{Array};
use numpy::{IntoPyArray, PyArray1};


pub mod simulator {
  use rand::rngs::StdRng;
  use rand::prelude::*;
  pub fn coverage_sim(n0: i64, p: f64, N: usize, num_trials: usize) -> Vec<u64>{
    let mut trial_times = vec![0; num_trials];
    let mut rng = StdRng::from_entropy();
    for trial in 0..num_trials {
      let mut visits = vec![0; N];
      let mut current_pos = n0;
      visits[current_pos as usize - 1] += 1;
      let mut time = 0;
      let mut num_visited: usize = visits.iter().sum();
      while num_visited != N {
        // draw a random number
        let val: f64 = rng.gen();
        // stay put
        if val < 1.0 - p {
          continue
        }
        // move to the left
        else if val < 1.0 - p/2.0 {
          
          current_pos -= 1
        }
        // move to the right
        else{
          
          current_pos += 1;
        }

        //  boundary checks
        if current_pos < 1 {
          current_pos += 1;
        }
        else if current_pos > N as i64{
          current_pos  -= 1;
        }
        // # increment visit
        visits[current_pos as usize - 1] = 1;
        num_visited = visits.iter().sum();

        // # increment time
        time += 1;

      }
      trial_times[trial] = time;
    }
    trial_times
  }

}




#[pymodule]
fn rust_demo(_py: Python, module: &PyModule) -> PyResult<()> {
  module.add_wrapped(wrap_pyfunction!(coverage_sim))?;
  Ok(())
}

#[pyfunction]
/// coverage(n0, N, p, num_trials)
/// --
/// blah blah
fn coverage_sim(py: Python, n0: i64, p: f64, N: i64, num_trials: i64) -> Py<PyArray1<u64>> {
  let data = simulator::coverage_sim(n0, p, N as usize, num_trials as usize);
  data.into_pyarray(py).to_owned()
}


