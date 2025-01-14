use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = _fast_graphql::stub_info()?;
    stub.generate()?;
    Ok(())
}
