use metabodecon::spectrum;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Spectrum {
    inner: spectrum::Spectrum,
}

impl AsRef<spectrum::Spectrum> for Spectrum {
    fn as_ref(&self) -> &spectrum::Spectrum {
        &self.inner
    }
}

#[pymethods]
impl Spectrum {
    #[new]
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
    ) -> PyResult<Self> {
        match spectrum::Spectrum::new(chemical_shifts, intensities, signal_boundaries) {
            Ok(spectrum) => Ok(Spectrum { inner: spectrum }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_bruker(
        path: &str,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
    ) -> PyResult<Self> {
        match spectrum::Bruker::read_spectrum(path, experiment, processing, signal_boundaries) {
            Ok(spectrum) => Ok(Spectrum { inner: spectrum }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_bruker_set(
        path: &str,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
    ) -> PyResult<Vec<Self>> {
        match spectrum::Bruker::read_spectra(path, experiment, processing, signal_boundaries) {
            Ok(spectra) => Ok(spectra
                .into_iter()
                .map(|spectrum| Spectrum { inner: spectrum })
                .collect()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_hdf5(path: &str, dataset: &str) -> PyResult<Self> {
        match spectrum::Hdf5::read_spectrum(path, dataset) {
            Ok(spectrum) => Ok(Spectrum { inner: spectrum }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_hdf5_set(path: &str) -> PyResult<Vec<Self>> {
        match spectrum::Hdf5::read_spectra(path) {
            Ok(spectra) => Ok(spectra
                .into_iter()
                .map(|spectrum| Spectrum { inner: spectrum })
                .collect()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn write_hdf5(path: &str, spectrum: &Spectrum) -> PyResult<()> {
        match spectrum::Hdf5::write_spectrum(path, spectrum.as_ref()) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn write_hdf5_set(path: &str, spectra: Vec<Spectrum>) -> PyResult<()> {
        match spectrum::Hdf5::write_spectra(path, &spectra) {
            Ok(_) => Ok(()),
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
    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.inner.signal_boundaries()
    }

    #[setter]
    pub fn set_chemical_shifts(
        &mut self,
        chemical_shifts: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        match self
            .inner
            .set_chemical_shifts(chemical_shifts.as_slice()?.to_vec())
        {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[setter]
    pub fn set_intensities(&mut self, intensities_raw: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        match self
            .inner
            .set_intensities(intensities_raw.as_slice()?.to_vec())
        {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[setter]
    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) -> PyResult<()> {
        match self
            .inner
            .set_signal_boundaries(signal_boundaries)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}
