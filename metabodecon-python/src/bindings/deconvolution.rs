use crate::bindings::Lorentzian;
use metabodecon::deconvolution;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Deconvolution {
    inner: deconvolution::Deconvolution,
}

impl From<deconvolution::Deconvolution> for Deconvolution {
    fn from(inner: deconvolution::Deconvolution) -> Self {
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
                .map(|l| (*l).into())
                .collect::<Vec<Lorentzian>>(),
        )
    }

    #[getter]
    pub fn mse(&self) -> f64 {
        self.inner.mse()
    }

    pub fn superposition(&self, chemical_shift: f64) -> f64 {
        deconvolution::Lorentzian::superposition(chemical_shift, self.inner.lorentzians())
    }

    pub fn superposition_vec<'py>(
        &self,
        py: Python<'py>,
        chemical_shifts: PyReadonlyArray1<'_, f64>,
    ) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(
            py,
            &deconvolution::Lorentzian::superposition_vec(
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
            &deconvolution::Lorentzian::par_superposition_vec(
                chemical_shifts.as_slice().unwrap(),
                self.inner.lorentzians(),
            ),
        )
    }
}
