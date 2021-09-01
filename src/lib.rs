#![feature(associated_type_defaults)]
#![feature(option_result_unwrap_unchecked)]

use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::ffi::Py_hash_t;
use pyo3::prelude::*;

mod big_int;
mod utils;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const BINARY_SHIFT: usize = (Digit::BITS - 1) as usize;

type BigInt = big_int::BigInt<Digit, BINARY_SHIFT>;

#[pyclass(module = "rithm", subclass)]
struct Int(BigInt);

#[pymethods]
impl Int {
    #[new]
    #[args(_string = "\"0\"", base = 10)]
    fn new(_string: &str, base: u8) -> PyResult<Self> {
        Ok(Int {
            0: match BigInt::new(_string, base) {
                Ok(value) => Ok(value),
                Err(reason) => Err(PyValueError::new_err(reason)),
            }?,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Int {
    fn __hash__(&self) -> PyResult<Py_hash_t> {
        Ok(self.0.hash() as Py_hash_t)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("rithm.Int('{}')", self.0.to_string(10)))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string(10))
    }
}

#[pymodule]
fn _rithm(_py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    module.setattr("__doc__", env!("CARGO_PKG_DESCRIPTION"))?;
    module.add_class::<Int>()?;
    Ok(())
}
