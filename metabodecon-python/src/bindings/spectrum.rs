use crate::MetabodeconError;
use crate::error::SerializationError;
use metabodecon::spectrum;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PyDict;

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

    #[staticmethod]
    pub fn read_jcampdx(path: &str, signal_boundaries: (f64, f64)) -> PyResult<Self> {
        match spectrum::JcampDx::read_spectrum(path, signal_boundaries) {
            Ok(spectrum) => Ok(spectrum.into()),
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

    #[getter]
    pub fn nucleus(&self) -> String {
        self.inner.nucleus().to_string()
    }

    #[getter]
    pub fn frequency(&self) -> f64 {
        self.inner.frequency()
    }

    #[getter]
    pub fn reference_compound<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        let reference = self.inner.reference_compound();
        dict.set_item("chemical_shift", reference.chemical_shift())?;
        dict.set_item("index", reference.index())?;
        match reference.name() {
            Some(name) => dict.set_item("name", name)?,
            None => dict.set_item("name", "unknown")?,
        };
        match reference.referencing_method() {
            Some(referencing_method) => {
                dict.set_item("referencing_method", referencing_method.to_string())?
            }
            None => dict.set_item("referencing_method", "unknown")?,
        };

        Ok(dict)
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

    #[setter]
    pub fn set_nucleus(&mut self, nucleus: &str) {
        self.inner.set_nucleus(nucleus);
    }

    #[setter]
    pub fn set_frequency(&mut self, frequency: f64) {
        self.inner.set_frequency(frequency);
    }

    #[setter]
    pub fn set_reference_compound(&mut self, reference: Bound<'_, PyDict>) -> PyResult<()> {
        let reference = reference.as_any();
        let chemical_shift = reference
            .get_item("chemical_shift")?
            .extract::<f64>()?;
        let index = reference.get_item("index")?.extract::<usize>()?;
        let name = match reference.get_item("name") {
            Ok(name) => name.extract::<String>().ok(),
            Err(_) => None,
        };
        let method = match reference.get_item("referencing_method") {
            Ok(method) => method
                .extract::<String>()
                .ok()
                .map(|method| std::str::FromStr::from_str(&method).unwrap()),
            Err(_) => None,
        };
        let reference = spectrum::meta::ReferenceCompound::new(chemical_shift, index, name, method);
        self.inner.set_reference_compound(reference);

        Ok(())
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
