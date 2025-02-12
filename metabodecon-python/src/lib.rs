use pyo3::prelude::*;

mod bindings;
pub(crate) use bindings::{Deconvoluter, Deconvolution, Lorentzian, Spectrum};

mod error;
pub(crate) use error::MetabodeconError;

#[pymodule]
fn _metabodecon(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<Deconvoluter>()?;
    m.add_class::<Deconvolution>()?;
    m.add_class::<Lorentzian>()?;
    m.add_class::<Spectrum>()?;

    let exceptions = MetabodeconError::error_module(py)?;
    m.add_submodule(&exceptions)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("metabodecon.exceptions", exceptions)?;

    Ok(())
}
