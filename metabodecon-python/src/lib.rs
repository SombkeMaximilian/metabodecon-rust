use pyo3::prelude::*;

mod bindings;
use bindings::{Deconvoluter, Deconvolution, Lorentzian, Spectrum};

#[pymodule]
fn _metabodecon(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Deconvoluter>()?;
    m.add_class::<Deconvolution>()?;
    m.add_class::<Lorentzian>()?;
    m.add_class::<Spectrum>()?;
    Ok(())
}
