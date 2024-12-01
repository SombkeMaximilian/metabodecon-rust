use metabodecon::Spectrum;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct MdSpectrum {
    pub inner: Spectrum,
}

#[pymethods]
impl MdSpectrum {
    #[new]
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Self {
        MdSpectrum {
            inner: Spectrum::new(
                chemical_shifts,
                intensities,
                signal_boundaries,
                water_boundaries,
            ),
        }
    }

    #[staticmethod]
    pub fn from_hdf5(path: &str, dataset: &str) -> PyResult<Self> {
        match Spectrum::from_hdf5(path, dataset) {
            Ok(spectrum) => Ok(MdSpectrum { inner: spectrum }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[getter]
    pub fn chemical_shifts<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, self.inner.chemical_shifts())
    }

    #[getter]
    pub fn intensities<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, self.inner.intensities())
    }

    #[getter]
    pub fn intensities_raw<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_slice(py, self.inner.intensities_raw())
    }

    #[getter]
    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.inner.signal_boundaries()
    }

    #[getter]
    pub fn water_boundaries(&self) -> (f64, f64) {
        self.inner.water_boundaries()
    }

    #[setter]
    pub fn set_chemical_shifts(&mut self, chemical_shifts: PyReadonlyArray1<'_, f64>) {
        self.inner
            .set_chemical_shifts(chemical_shifts.as_slice().unwrap().to_vec());
    }

    #[setter]
    pub fn set_intensities(&mut self, intensities: PyReadonlyArray1<'_, f64>) {
        self.inner
            .set_intensities(intensities.as_slice().unwrap().to_vec());
    }

    #[setter]
    pub fn set_intensities_raw(&mut self, intensities_raw: PyReadonlyArray1<'_, f64>) {
        self.inner
            .set_intensities_raw(intensities_raw.as_slice().unwrap().to_vec());
    }

    #[setter]
    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) {
        self.inner.set_signal_boundaries(signal_boundaries);
    }

    #[setter]
    pub fn set_water_boundaries(&mut self, water_boundaries: (f64, f64)) {
        self.inner.set_water_boundaries(water_boundaries);
    }
}
