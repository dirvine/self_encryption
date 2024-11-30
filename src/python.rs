use pyo3::prelude::*;
use bytes::Bytes;
use crate::{
    DataMap,
    XorName,
    Error,
    encrypt_from_file as rust_encrypt_from_file,
    decrypt_from_storage as rust_decrypt_from_storage,
    streaming_decrypt_from_storage as rust_streaming_decrypt_from_storage,
};

// Make DataMap usable from Python
#[pyclass]
#[derive(Clone)]
pub struct PyDataMap {
    inner: DataMap,
}


#[pymethods]
impl PyDataMap {
    #[new]
    pub fn new() -> Self {
        PyDataMap { 
            inner: DataMap::new(vec![]) 
        }
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    #[staticmethod]
    pub fn from_json(json_str: &str) -> PyResult<Self> {
        serde_json::from_str(json_str)
            .map(|inner| PyDataMap { inner })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

impl From<DataMap> for PyDataMap {
    fn from(inner: DataMap) -> Self {
        PyDataMap { inner }
    }
}

// Make XorName usable from Python
#[pyclass]
#[derive(Clone)]
pub struct PyXorName {
    inner: XorName,
}

impl PyXorName {
    pub fn new(inner: XorName) -> Self {
        PyXorName { inner }
    }
}

#[pymethods]
impl PyXorName {
    #[new]
    fn py_new(bytes: &[u8]) -> PyResult<Self> {
        if bytes.len() != 32 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "XorName must be exactly 32 bytes"
            ));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(bytes);
        Ok(PyXorName {
            inner: XorName(array)
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(hex::encode(self.inner.0))
    }
}

impl From<XorName> for PyXorName {
    fn from(inner: XorName) -> Self {
        PyXorName { inner }
    }
}

// Create a Python tuple type for our return value
#[pyclass]
pub struct EncryptResult {
    #[pyo3(get)]
    data_map: PyDataMap,
    #[pyo3(get)]
    names: Vec<PyXorName>,
}

#[pymodule]
fn _self_encryption(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Register classes
    m.add_class::<PyDataMap>()?;
    m.add_class::<PyXorName>()?;
    m.add_class::<EncryptResult>()?;

    // Register constants
    m.setattr("MIN_CHUNK_SIZE", 1i32)?;
    m.setattr("MIN_ENCRYPTABLE_BYTES", 3i32)?;
    m.setattr("MAX_CHUNK_SIZE", 1024 * 1024i32)?;
    m.setattr("COMPRESSION_QUALITY", 6i32)?;

    // Register functions
    m.add_function(wrap_pyfunction!(encrypt_from_file, py)?)?;
    m.add_function(wrap_pyfunction!(decrypt_from_storage, py)?)?;
    m.add_function(wrap_pyfunction!(streaming_decrypt_from_storage, py)?)?;

    Ok(())
}

#[pyfunction]
fn encrypt_from_file(file_path: &str, output_dir: &str) -> PyResult<EncryptResult> {
    let path = std::path::Path::new(file_path);
    let out_path = std::path::Path::new(output_dir);
    rust_encrypt_from_file(path, out_path)
        .map(|(data_map, names)| EncryptResult {
            data_map: PyDataMap::from(data_map),
            names: names.into_iter().map(PyXorName::from).collect(),
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn decrypt_from_storage(data_map: &PyDataMap, output_file: &str, chunks_dir: &str) -> PyResult<()> {
    let out_path = std::path::Path::new(output_file);
    let chunks_path = std::path::Path::new(chunks_dir);
    rust_decrypt_from_storage(&data_map.inner, out_path, |hash| {
        let chunk_path = chunks_path.join(hex::encode(hash));
        std::fs::read(chunk_path)
            .map(Bytes::from)
            .map_err(|e| Error::Generic(e.to_string()))
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn streaming_decrypt_from_storage(data_map: &PyDataMap, output_file: &str, chunks_dir: &str) -> PyResult<()> {
    let out_path = std::path::Path::new(output_file);
    let chunks_path = std::path::Path::new(chunks_dir);
    rust_streaming_decrypt_from_storage(&data_map.inner, out_path, |hashes| {
        hashes.iter().map(|hash| {
            let chunk_path = chunks_path.join(hex::encode(hash));
            std::fs::read(chunk_path)
                .map(Bytes::from)
                .map_err(|e| Error::Generic(e.to_string()))
        }).collect()
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
