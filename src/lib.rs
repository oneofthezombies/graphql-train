use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

/// Formats the sum of two numbers as string.
#[gen_stub_pyfunction]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

async fn inner(a: &str, b: &str) -> String {
    format!("{a}{b}")
}

#[gen_stub_pyfunction]
#[pyfunction]
async fn concat(a: String, b: String) -> PyResult<String> {
    Ok(inner(&a, &b).await)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _fast_graphql(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(concat, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
