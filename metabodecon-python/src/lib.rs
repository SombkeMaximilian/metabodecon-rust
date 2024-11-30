use pyo3::prelude::*;

mod bindings;

use bindings::MdSpectrum;

#[pymodule]
fn metabodecon_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MdSpectrum>()?;
    Ok(())
}
