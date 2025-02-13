use crate::MetabodeconError;
use crate::error::SerializationError;
use metabodecon::spectrum;
use numpy::PyArray1;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Spectrum {
    inner: spectrum::Spectrum,
}

impl AsRef<spectrum::Spectrum> for Spectrum {
    fn as_ref(&self) -> &spectrum::Spectrum {
        &self.inner
    }
}

impl From<spectrum::Spectrum> for Spectrum {
    fn from(value: spectrum::Spectrum) -> Self {
        Spectrum { inner: value }
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
            Ok(spectrum) => Ok(spectrum.into()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    #[staticmethod]
    pub fn read_bruker(
        path: &str,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
    ) -> PyResult<Self> {
        match spectrum::Bruker::read_spectrum(path, experiment, processing, signal_boundaries) {
            Ok(spectrum) => Ok(spectrum.into()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    #[staticmethod]
    pub fn read_bruker_set(
        path: &str,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
    ) -> PyResult<Vec<Self>> {
        match spectrum::Bruker::read_spectra(path, experiment, processing, signal_boundaries) {
            Ok(spectra) => Ok(spectra
                .into_iter()
                .map(|spectrum| spectrum.into())
                .collect()),
            Err(e) => Err(MetabodeconError::from(e).into()),
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
    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) -> PyResult<()> {
        match self
            .inner
            .set_signal_boundaries(signal_boundaries)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn write_json(&self, path: &str) -> PyResult<()> {
        let serialized = match serde_json::to_string_pretty(self.as_ref()) {
            Ok(serialized) => serialized,
            Err(error) => return Err(SerializationError::new_err(error.to_string())),
        };
        std::fs::write(path, serialized)?;

        Ok(())
    }

    #[staticmethod]
    pub fn read_json(path: &str) -> PyResult<Self> {
        let serialized = std::fs::read_to_string(path)?;
        match serde_json::from_str::<spectrum::Spectrum>(&serialized) {
            Ok(deserialized) => Ok(deserialized.into()),
            Err(error) => Err(SerializationError::new_err(error.to_string())),
        }
    }

    pub fn write_bin(&self, path: &str) -> PyResult<()> {
        let serialized = match rmp_serde::to_vec(self.as_ref()) {
            Ok(serialized) => serialized,
            Err(error) => return Err(SerializationError::new_err(error.to_string())),
        };
        std::fs::write(path, serialized)?;

        Ok(())
    }

    #[staticmethod]
    pub fn read_bin(path: &str) -> PyResult<Self> {
        let serialized = std::fs::read(path)?;

        match rmp_serde::from_slice::<spectrum::Spectrum>(&serialized) {
            Ok(deserialized) => Ok(deserialized.into()),
            Err(error) => Err(SerializationError::new_err(error.to_string())),
        }
    }
}
