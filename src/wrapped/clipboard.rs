use std::rc::Rc;

use iced::Clipboard;
use parking_lot::Mutex;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedClipboard>()?;
    Ok(())
}

/// A buffer for short-term storage and transfer within and between applications.
///
/// Warning
/// -------
/// The clipboard is only valid during the call to :meth:`pyiced.IcedApp.update()`.
///
/// Calling :meth:`pyiced.Clipboard.read()` or :meth:`pyiced.Clipboard.write()` in another thread will kill the application!
///
/// See also
/// --------
/// `iced::Clipboard <https://docs.rs/iced/0.3.0/iced/struct.Clipboard.html>`_
#[pyclass(name = "Clipboard", module = "pyiced", unsendable, freelist = 3)]
pub(crate) struct WrappedClipboard(pub Rc<Mutex<Option<*mut Clipboard>>>);

#[pymethods]
impl WrappedClipboard {
    /// read($self, /)
    /// --
    ///
    /// Reads the current content of the clipboard as text.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     The current contents of the clipboard.
    fn read(&self) -> PyResult<Option<String>> {
        let rc: &Mutex<_> = self.0.as_ref();
        let guard = match rc.try_lock() {
            None => return Err(PyRuntimeError::new_err("Clipboard in use.")),
            Some(guard) => guard,
        };
        let clipboard = match *guard {
            None => return Err(PyRuntimeError::new_err("Clipboard expired.")),
            Some(clipboard) => unsafe { &mut *clipboard },
        };
        Ok(clipboard.read())
    }

    /// write($self, /, value)
    /// --
    ///
    /// Writes the given text contents to the clipboard.
    ///
    /// Arguments
    /// ---------
    /// value : str
    ///     The new contents of the clipboard.
    fn write(&self, value: String) -> PyResult<()> {
        let rc: &Mutex<_> = self.0.as_ref();
        let guard = match rc.try_lock() {
            None => return Err(PyRuntimeError::new_err("Clipboard in use.")),
            Some(guard) => guard,
        };
        let clipboard = match *guard {
            None => return Err(PyRuntimeError::new_err("Clipboard expired.")),
            Some(clipboard) => unsafe { &mut *clipboard },
        };
        clipboard.write(value);
        Ok(())
    }
}
