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

#[pyclass]
struct Schema {
    inner: Arc<dynamic::Schema>,
}

#[pymethods]
impl Schema {
    #[new]
    fn new() -> PyResult<Self> {
        let builder = dynamic::Schema::build(
            r"{
            type Query {
                hello: String!
            }
        }",
            None,
            None,
        );
        let query = Object::new("Query").field(Field::new(
            "hello",
            TypeRef::named_nn(TypeRef::STRING),
            |_ctx| FieldFuture::new(async { Ok(Some(Value::from("Hello, FastGraphQL!"))) }),
        ));
        let builder = builder.register(query);
        let schema = builder
            .finish()
            .map_err(|e| PyException::new_err(format!("{}", e)))?;
        Ok(Self {
            inner: Arc::new(schema),
        })
    }

    fn execute<'py>(&self, request: &Bound<'py, PyString>) -> PyResult<Bound<PyAny>> {
        let a = request.to_string();
        future_into_py(request.py(), async {
            let response = self.inner.execute(a).await;
            let a = format!("{:?}", response);
            Python::with_gil(|py| {
                let b: Bound<'py, PyString> = PyString::new(request.py(), a.as_str());
                Ok(b)
            })
        })
    }
}

#[pymodule]
fn _fast_graphql(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Schema>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
