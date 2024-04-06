/// Mutably edit immutable Python tuples
use pyo3::prelude::{pyfunction, pymodule, wrap_pyfunction};
use pyo3::types::{PyAny, PyModule, PyTuple};
use pyo3::{exceptions, ffi, Bound, PyResult};

/// Replace an item in a Python tuple.
#[pyfunction]
fn setitem(tuple: &Bound<'_, PyTuple>, idx: isize, obj: &PyAny) -> PyResult<()> {
    let is_tuple = unsafe { ffi::PyTuple_Check(tuple.as_ptr()) };
    if is_tuple == 0 {
        return Err(exceptions::PyTypeError::new_err(
            "mutuple.setitem() argument 0 must be a tuple",
        ));
    }
    let size = unsafe { ffi::PyTuple_Size(tuple.as_ptr()) };
    if idx < 0 || idx >= size {
        let error_msg = format!(
            "mutuple.setitem(_, {idx}, _) index out of range; index must be less than {size}."
        );
        return Err(exceptions::PyIndexError::new_err(error_msg));
    }
    unsafe {
        // PyTuple_SetItem steals a reference.
        ffi::Py_INCREF(obj.as_ptr());

        let refcount = ffi::Py_REFCNT(tuple.as_ptr());
        // This is a hack, but it works. PyTuple_SetItem only allows setting items on tuples that
        // have no external references. We temporarily lower the reference count to get past the
        // check in PyTuple_SetItem(). yolo.
        while ffi::Py_REFCNT(tuple.as_ptr()) > 1 {
            ffi::Py_DECREF(tuple.as_ptr());
        }

        // We could use PyTuple_SET_ITEM() but it's not part of the stable API, hence this hack.
        let ok = ffi::PyTuple_SetItem(tuple.as_ptr(), idx, obj.as_ptr());
        if ok != 0 {
            ffi::PyErr_Clear();
        }

        // Restore the original reference count.
        while ffi::Py_REFCNT(tuple.as_ptr()) < refcount {
            ffi::Py_INCREF(tuple.as_ptr());
        }

        if ok != 0 {
            return Err(exceptions::PyValueError::new_err(
                "mutuple.setitem() failed",
            ));
        }
    }

    Ok(())
}

/// mutuple edits "immutable" tuples.
#[pymodule]
fn mutuple(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(setitem, module)?)?;
    Ok(())
}
