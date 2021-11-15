use iced::Element;
use pyo3::prelude::*;
use pyo3::PyGCProtocol;

use crate::common::{debug_str, GCProtocol, Message, ToNative};

macro_rules! init_mod {
    ($($name:ident($module:ident -> $typ:ident)),+ $(,)?) => {
        $( mod $module; )*

        $( pub(crate) use $module :: $typ; )*

        pub(crate) fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            m.add_class::<WrappedWidgetBuilder>()?;
            $( $module::init_mod(py, m)?; )*
            Ok(())
        }

        #[derive(Debug, Clone)]
        pub(crate) enum WidgetBuilder {
            $( $name($typ) ),+
        }

        $(
            impl From<$typ> for WidgetBuilder {
                fn from(value: $typ) -> WidgetBuilder {
                    WidgetBuilder::$name(value)
                }
            }

            impl From<$typ> for WrappedWidgetBuilder {
                fn from(value: $typ) -> WrappedWidgetBuilder {
                    WrappedWidgetBuilder(WidgetBuilder::$name(value))
                }
            }
        )+

        impl ToNative for WidgetBuilder {
            fn to_native(&self, py: Python) -> Element<'static, Message> {
                match self {
                    $( WidgetBuilder::$name(value) => value.to_native(py) ),*
                }
            }
        }

        #[pymethods]
        impl WrappedWidgetBuilder {
            fn __str__(&self) -> PyResult<String> {
                match &self.0 {
                    $( WidgetBuilder::$name(value) => debug_str(value) ),+
                }
            }
        }

        impl GCProtocol for WidgetBuilder {
            fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
                match self {
                    $( WidgetBuilder::$name(value) => value.traverse(visit) ),+
                }
            }
        }
    };
}

init_mod!(
    NoElement(no_element -> NoElementBuilder),
    Button(button -> ButtonBuilder),
    // TODO: Canvas
    Checkbox(checkbox -> CheckboxBuilder),
    Column(column -> ColumnBuilder),
    Container(container -> ContainerBuilder),
    Image(image -> ImageBuilder),
    // TODO: PaneGrid
    PickList(pick_list -> PickListBuilder),
    ProgressBar(progress_bar -> ProgressBarBuilder),
    Radio(radio -> RadioBuilder),
    Row(row -> RowBuilder),
    Rule(rule -> RuleBuilder),
    Scrollable(scrollable -> ScrollableBuilder),
    Slider(slider -> SliderBuilder),
    Space(space -> SpaceBuilder),
    Svg(svg -> SvgBuilder),
    Text(text -> TextBuilder),
    TextInput(text_input -> TextInputBuilder),
    Tooltip(tooltip -> TooltipBuilder),
);

#[pyclass(name = "Element", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedWidgetBuilder(pub WidgetBuilder);

impl From<WidgetBuilder> for WrappedWidgetBuilder {
    fn from(value: WidgetBuilder) -> WrappedWidgetBuilder {
        Self(value)
    }
}

impl Default for WidgetBuilder {
    fn default() -> Self {
        Self::NoElement(NoElementBuilder::default())
    }
}

#[pyproto]
impl PyGCProtocol for WrappedWidgetBuilder {
    fn __traverse__(&self, visit: pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.0.traverse(&visit)?;
        Ok(())
    }

    fn __clear__(&mut self) {
        self.0 = Default::default();
    }
}
