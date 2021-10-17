use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::{Message, ToNative, debug_str};

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

        #[pyclass(name="Element", module="pyiced.pyiced")]
        #[derive(Debug, Clone)]
        pub(crate) struct WrappedWidgetBuilder(pub WidgetBuilder);

        impl From<WidgetBuilder> for WrappedWidgetBuilder {
            fn from(value: WidgetBuilder) -> WrappedWidgetBuilder {
                Self(value)
            }
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
            fn to_native(&self, py: Python) -> iced::Element<'static, Message> {
                match self {
                    $( WidgetBuilder::$name(value) => value.to_native(py) ),*
                }
            }
        }

        #[pyproto]
        impl PyObjectProtocol for WrappedWidgetBuilder {
            fn __str__(&self) -> PyResult<String> {
                match &self.0 {
                    $( WidgetBuilder::$name(value) => debug_str(value) ),+
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
