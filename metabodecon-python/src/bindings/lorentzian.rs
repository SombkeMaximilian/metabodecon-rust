use metabodecon::Lorentzian;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct MdLorentzian {
    pub inner: Lorentzian,
}

#[pymethods]
impl MdLorentzian {
    #[new]
    pub fn new(sfhw: f64, hw2: f64, maxp: f64) -> Self {
        MdLorentzian {
            inner: Lorentzian::new(sfhw, hw2, maxp),
        }
    }
}
