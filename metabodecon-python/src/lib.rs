use pyo3::prelude::*;

mod bindings;

use bindings::{Deconvolution, Lorentzian, Spectrum};

#[pymodule]
fn metabodecon_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Deconvolution>()?;
    m.add_class::<Lorentzian>()?;
    m.add_class::<Spectrum>()?;
    Ok(())
}
