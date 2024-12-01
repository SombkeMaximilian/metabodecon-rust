use metabodecon::Lorentzian;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct MdLorentzian {
    inner: Lorentzian,
}

#[pymethods]
impl MdLorentzian {
    #[new]
    pub fn new(sf: f64, hw: f64, maxp: f64) -> Self {
        MdLorentzian {
            inner: Lorentzian::new(sf * hw, hw.powi(2), maxp),
        }
    }

    #[staticmethod]
    pub fn from_transformed(sfhw: f64, hw2: f64, maxp: f64) -> Self {
        MdLorentzian {
            inner: Lorentzian::new(sfhw, hw2, maxp),
        }
    }

    #[getter]
    pub fn sf(&self) -> f64 {
        self.inner.sf()
    }

    #[getter]
    pub fn hw(&self) -> f64 {
        self.inner.hw()
    }

    #[getter]
    pub fn maxp(&self) -> f64 {
        self.inner.maxp()
    }

    #[getter]
    pub fn parameters(&self) -> (f64, f64, f64) {
        self.inner.retransformed_parameters()
    }

    #[setter]
    pub fn set_sf(&mut self, sf: f64) {
        self.inner.set_sf(sf);
    }

    #[setter]
    pub fn set_hw(&mut self, hw: f64) {
        self.inner.set_hw(hw);
    }

    #[setter]
    pub fn set_maxp(&mut self, maxp: f64) {
        self.inner.set_maxp(maxp);
    }

    #[setter]
    pub fn set_parameters(&mut self, parameters: (f64, f64, f64)) {
        self.inner
            .set_retransformed_parameters(parameters.0, parameters.1, parameters.2);
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.inner.evaluate(x)
    }

    pub fn evaluate_vec<'py>(
        &self,
        py: Python<'py>,
        x: PyReadonlyArray1<'_, f64>,
    ) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, &self.inner.evaluate_vec(x.as_slice().unwrap()))
    }

    #[staticmethod]
    pub fn superposition(x: f64, lorentzians: Vec<MdLorentzian>) -> f64 {
        let lorentzians = lorentzians.iter().map(|l| l.inner).collect::<Vec<_>>();
        Lorentzian::superposition(x, &lorentzians)
    }

    #[staticmethod]
    pub fn superposition_vec<'py>(
        py: Python<'py>,
        x: PyReadonlyArray1<'_, f64>,
        lorentzians: Vec<MdLorentzian>,
    ) -> Bound<'py, PyArray1<f64>> {
        let lorentzians = lorentzians.iter().map(|l| l.inner).collect::<Vec<_>>();
        PyArray1::from_slice(
            py,
            &Lorentzian::superposition_vec(x.as_slice().unwrap(), &lorentzians),
        )
    }

    #[staticmethod]
    pub fn par_superposition_vec<'py>(
        py: Python<'py>,
        x: PyReadonlyArray1<'_, f64>,
        lorentzians: Vec<MdLorentzian>,
    ) -> Bound<'py, PyArray1<f64>> {
        let lorentzians = lorentzians.iter().map(|l| l.inner).collect::<Vec<_>>();
        PyArray1::from_slice(
            py,
            &Lorentzian::par_superposition_vec(x.as_slice().unwrap(), &lorentzians),
        )
    }
}
