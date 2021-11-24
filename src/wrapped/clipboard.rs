use std::rc::Weak;

use iced::Clipboard;
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
/// See also
/// --------
/// `iced::Clipboard <https://docs.rs/iced/0.3.0/iced/struct.Clipboard.html>`_
#[pyclass(name = "Clipboard", module = "pyiced", unsendable, freelist = 3)]
pub(crate) struct WrappedClipboard(pub Weak<*mut Clipboard>);

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
        match self.0.upgrade() {
            Some(clipboard) => Ok(unsafe { &**clipboard }.read()),
            None => Err(PyRuntimeError::new_err("Clipboard expired.")),
        }
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
        match self.0.upgrade() {
            Some(clipboard) => {
                unsafe { &mut **clipboard }.write(value);
                Ok(())
            },
            None => Err(PyRuntimeError::new_err("Clipboard expired.")),
        }
    }
}
