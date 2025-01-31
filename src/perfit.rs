use pyo3::prelude::*;

use crate::python::par_iter_obj_collection_by_ref::{StateMachine, Transition};

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, otherwise Python will not be able to
/// import the module.
#[pymodule]
fn perfit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StateMachine>()?;
    m.add_class::<Transition>()?;
    Ok(())
}
