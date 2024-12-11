use crate::bindings::Lorentzian;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Deconvolution {
    inner: metabodecon::Deconvolution,
}

impl Deconvolution {
    pub fn from_inner(inner: metabodecon::Deconvolution) -> Self {
        Deconvolution { inner }
    }
}

#[pymethods]
impl Deconvolution {
    #[getter]
    pub fn lorentzians<'py>(&self, py: Python<'py>) -> Result<Bound<'py, PyList>, PyErr> {
        PyList::new(
            py,
            self.inner
                .lorentzians()
                .iter()
                .map(|l| Lorentzian::from_inner(*l))
                .collect::<Vec<_>>(),
        )
    }

    #[getter]
    pub fn mse(&self) -> f64 {
        self.inner.mse()
    }

    pub fn superposition(&self, chemical_shift: f64) -> f64 {
        metabodecon::Lorentzian::superposition(chemical_shift, self.inner.lorentzians())
    }

    pub fn superposition_vec<'py>(
        &self,
        py: Python<'py>,
        chemical_shifts: PyReadonlyArray1<'_, f64>,
    ) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(
            py,
            &metabodecon::Lorentzian::superposition_vec(
                chemical_shifts.as_slice().unwrap(),
                self.inner.lorentzians(),
            ),
        )
    }

    pub fn par_superposition_vec<'py>(
        &self,
        py: Python<'py>,
        chemical_shifts: PyReadonlyArray1<'_, f64>,
    ) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(
            py,
            &metabodecon::Lorentzian::par_superposition_vec(
                chemical_shifts.as_slice().unwrap(),
                self.inner.lorentzians(),
            ),
        )
    }
}
