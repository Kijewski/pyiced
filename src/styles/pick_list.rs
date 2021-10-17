#![allow(clippy::needless_option_as_deref)]

use iced::pick_list::{Menu, Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::wrapped::WrappedColor;
use crate::{dyn_style_proto, extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPickListMenu>()?;
    m.add_class::<WrappedPickListStyle>()?;
    m.add_class::<WrappedPickListStyleSheet>()?;
    Ok(())
}

/// PickListStyle(proto=None, **kwargs)
/// --
///
/// The appearance of a pick list for some state.
///
/// Parameters
/// ----------
/// proto : Optional[Union[PickListStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active" and "hovered",
///     same as the argument for :class:`~pyiced.PickListStyleSheet`.
///
///     None is the same as "active".
/// text_color : Color
///     The pick list's foreground color.
/// background : Color
///     The pick list's background color.
/// border_radius : float
///     The pick list's border radius.
/// border_width : float
///     The pick list's border width.
/// border_color : Color
///     The pick list's border color.
/// icon_size : float
///     The pick list's arrow down icon size.
///
/// See also
/// --------
/// `iced::widget::pick_list::Style <https://docs.rs/iced/0.3.0/iced/widget/pick_list/struct.Style.html>`_
#[pyclass(name = "PickListStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedPickListStyle(pub PickListStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct PickListStyle(pub Style);

#[pymethods]
impl WrappedPickListStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = dyn_style_proto!(proto, active, hovered);
        extract_multiple!(
            kwargs,
            PickListStyle(proto),
            text_color,
            background,
            border_radius,
            border_width,
            border_color,
            icon_size
        )
    }
}

getters! {
    WrappedPickListStyle => |&WrappedPickListStyle(PickListStyle(ref o))| o,
    text_color -> "Color" WrappedColor,
    background -> "Color" WrappedColor,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
    icon_size -> "float" f32,
}

/// PickListStyleSheet(menu, active, hovered=None)
/// --
///
/// The appearance of a pick list.
///
/// Parameters
/// ----------
/// menu : PickListMenu
///     Style of the drop down menu.
/// active : PickListStyle
///     Normal style of the pick list.
/// hovered : Optional[PickListStyle]
///     Style of the pick list when the cursor is hovering over it. Defaults to "active".
///
/// See also
/// --------
/// `iced::widget::pick_list::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/pick_list/trait.StyleSheet.html>`_
#[pyclass(name = "PickListStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedPickListStyleSheet(pub PickListStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct PickListStyleSheet {
    menu: Menu,
    active: Style,
    hovered: Style,
}

getters! {
    WrappedPickListStyleSheet => |&WrappedPickListStyleSheet(ref o)| o,
    menu -> "PickListMenu" WrappedPickListMenu,
    active -> "PickListStyle" WrappedPickListStyle,
    hovered -> "PickListStyle" WrappedPickListStyle,
}

#[pymethods]
impl WrappedPickListStyleSheet {
    #[new]
    fn new(
        menu: &WrappedPickListMenu,
        active: &WrappedPickListStyle,
        hovered: Option<&WrappedPickListStyle>,
    ) -> Self {
        let menu = menu.0.0;
        let active = active.0.0;
        let hovered = hovered.map_or(active, |s| s.0.0);
        Self(PickListStyleSheet {
            menu,
            active,
            hovered,
        })
    }
}

impl StyleSheet for PickListStyleSheet {
    fn menu(&self) -> Menu {
        self.menu
    }

    fn active(&self) -> Style {
        self.active
    }

    fn hovered(&self) -> Style {
        self.hovered
    }
}

/// PickListMenu(proto=None, **kwargs)
/// --
///
/// The appearance of a pick list menu.
///
/// Parameters
/// ----------
/// proto : Optional[PickListMenu]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
/// text_color : Color
///     The text color of the menu.
/// background : Color
///     The background color of the menu.
/// border_width : float
///     The border width of the menu.
/// border_color : Color
///     The border color of the menu.
/// selected_text_color : Color
///     The text color of the selected element.
/// selected_background : Color
///     Text background color of the selected element.
///
/// See also
/// --------
/// `iced::widget::pick_list::Menu <https://docs.rs/iced/0.3.0/iced/widget/pick_list/struct.Menu.html>`_
#[pyclass(name = "PickListMenu", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedPickListMenu(pub PickListMenu);

#[derive(Debug, Clone, Copy)]
pub(crate) struct PickListMenu(pub Menu);

#[pymethods]
impl WrappedPickListMenu {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&Self>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = proto.map_or_else(Menu::default, |p| p.0.0);
        extract_multiple!(
            kwargs,
            PickListMenu(proto),
            text_color,
            background,
            border_width,
            border_color,
            selected_text_color,
            selected_background
        )
    }
}

getters! {
    WrappedPickListMenu => |&WrappedPickListMenu(PickListMenu(ref o))| o,
    text_color -> "Color" WrappedColor,
    background -> "Color" WrappedColor,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
    selected_text_color -> "Color" WrappedColor,
    selected_background -> "Color" WrappedColor,
}
