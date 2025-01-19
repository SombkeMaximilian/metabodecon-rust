use metabodecon::spectrum;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Spectrum {
    inner: spectrum::Spectrum,
}

impl Spectrum {
    pub fn inner_mut(&mut self) -> &mut spectrum::Spectrum {
        &mut self.inner
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
        let reader = spectrum::BrukerReader::new();
        match reader.read_spectrum(path, experiment, processing, signal_boundaries) {
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
        let reader = spectrum::BrukerReader::new();
        match reader.read_spectra(path, experiment, processing, signal_boundaries) {
            Ok(spectra) => Ok(spectra
                .into_iter()
                .map(|spectrum| Spectrum { inner: spectrum })
                .collect()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_hdf5(path: &str, dataset: &str) -> PyResult<Self> {
        let reader = spectrum::Hdf5Reader::new();
        match reader.read_spectrum(path, dataset) {
            Ok(spectrum) => Ok(Spectrum { inner: spectrum }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_hdf5_set(path: &str) -> PyResult<Vec<Self>> {
        let reader = spectrum::Hdf5Reader::new();
        match reader.read_spectra(path) {
            Ok(spectra) => Ok(spectra
                .into_iter()
                .map(|spectrum| Spectrum { inner: spectrum })
                .collect()),
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
