use std::sync::Arc;

use async_graphql::dynamic;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_graphql::Value;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use pyo3_stub_gen::derive::gen_stub_pymethods;

#[gen_stub_pyclass]
#[pyclass]
struct Schema {
    inner: Arc<dynamic::Schema>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Schema {
    #[new]
    fn new() -> PyResult<Self> {
        let query = Object::new("Query").field(Field::new(
            "hello",
            TypeRef::named_nn(TypeRef::STRING),
            |_ctx| FieldFuture::new(async { Ok(Some(Value::from("Hello, FastGraphQL!"))) }),
        ));
        let builder = dynamic::Schema::build(query.type_name(), None, None).register(query);
        let schema = builder
            .finish()
            .map_err(|e| PyException::new_err(format!("{}", e)))?;
        Ok(Self {
            inner: Arc::new(schema),
        })
    }

    fn execute<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyString>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request_str = request.to_string();
        let schema = self.inner.clone();
        future_into_py(py, async move {
            let response = schema.execute(request_str).await;
            let debug_str = format!("{:?}", response);
            Python::with_gil(|py| Ok(PyString::new(py, &debug_str).to_object(py)))
        })
    }
}

#[pymodule]
fn _fast_graphql(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Schema>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
