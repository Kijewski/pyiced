from functools import wraps
from math import isnan, isinf, isfinite
from pathlib import Path
from typing import Annotated, Callable, Iterable, Optional, Tuple, Union, get_args, get_origin, get_type_hints


class _CheckableType:
    def check(self, name, value):
        raise NotImplemented


class _ValueRange(_CheckableType):
    def __init__(self, values: range):
        self.values = values

    def check(self, name, value):
        if value not in self.values:
            raise ValueError(
                f'Parameter {name:r} must have a value in {self.values}, '
                f'so the value {value:r} is invalid'
            )


class _NonNan(_CheckableType):
    def check(self, name, value):
        if isnan(value):
            raise ValueError(
                f'Parameter {name:r} must not have NaN value, '
                f'so the value {value:r} is invalid.'
            )


class _NonInf(_CheckableType):
    def check(self, name, value):
        if isinf(value):
            raise ValueError(
                f'Parameter {name:r} must not have an infinite value, '
                f'so the value {value:r} is invalid.'
            )


class _Finite(_CheckableType):
    def check(self, name, value):
        if not isfinite(value):
            raise ValueError(
                f'Parameter {name:r} must be infite value, '
                f'so the value {value:r} is invalid.'
            )


def check_annotations(func):
    @wraps(func)
    def wrapped(**kwargs):
        type_hints = get_type_hints(func, include_extras=True)
        for param, hint in type_hints.items():
            if get_origin(hint) is Annotated:
                _, *hint_args = get_args(hint)
                for arg in hint_args:
                    if isinstance(arg, _CheckableType):
                        arg.check(param, kwargs[param])
        return func(**kwargs)
    return wrapped


_BytesLike = Union[bytes, bytearray, memoryview]

_I8 = Annotated[int, _ValueRange(range(-0x80, 0x80))]
_I16 = Annotated[int, _ValueRange(range(-0x8000, 0x8000))]
_I32 = Annotated[int, _ValueRange(range(-0x8000_0000, 0x8000_0000))]
_I64 = Annotated[int, _ValueRange(range(-0x8000_0000_0000_0000, 0x8000_0000_0000_0000))]

_U8 = Annotated[int, _ValueRange(range(0x100))]
_U16 = Annotated[int, _ValueRange(range(0x1000))]
_U32 = Annotated[int, _ValueRange(range(0x1_0000_0000))]
_U64 = Annotated[int, _ValueRange(range(0x1_0000_0000_0000_0000))]

_FloatNonNan = Annotated[float, _NonNan()]
_FloatNonInf = Annotated[float, _NonInf()]
_FloatFinite = Annotated[float, _Finite()]


###################################################################################################
### Wrapped #######################################################################################
###################################################################################################


class Color:
    '''A color in the sRGB color space.'''

    def __init__(
        self,
        r: _FloatNonNan,
        g: _FloatNonNan,
        b: _FloatNonNan,
        a: Optional[_FloatNonNan]=None,
    ) -> Color:
        ...

    @property
    def r(self) -> float:
        '''Red component, 0.0 – 1.0'''

    @property
    def g(self) -> float:
        '''Green component, 0.0 – 1.0'''

    @property
    def b(self) -> float:
        '''Blue component, 0.0 – 1.0'''

    @property
    def a(self) -> float:
        '''Alpha channel, 0.0 – 1.0 (0.0 = transparent; 1.0 = opaque)'''

    BLACK: Color
    '''Color(0, 0, 0)'''

    WHITE: Color
    '''Color(1, 1, 1)'''

    TRANSPARENT: Color
    '''Color(0, 0, 0, a=0)'''


class Length:
    '''The strategy used to fill space in a specific dimension.'''

    @staticmethod
    def fill_portion(i: _U16) -> Length:
        '''Fill a portion of the remaining space relative to other elements.'''

    @staticmethod
    def units(i: _U16) -> Length:
        '''Fill a fixed amount of space.'''

    FILL: Length
    '''Fill all the remaining space.'''

    SHRINK: Length
    '''Fill the least amount of space.'''


class Align:
    '''Alignment on an axis of a container.'''

    START: Align
    '''Align at the start of the axis.'''

    CENTER: Align
    '''Align at the center of the axis.'''

    END: Align
    '''Align at the end of the axis.'''


class Font:
    '''A font.'''

    def __init__(self, name: str, data: _BytesLike) -> Font:
        ...

    @property
    def name(self) -> Optional[str]:
        '''The '(set, copied or defaulted) 'name' parameter given to the constructor.'''


class Element:
    '''TODO'''


# TODO: __init__.py


###################################################################################################
### Button ########################################################################################
###################################################################################################


class ButtonState:
    '''The state of a button().'''

    def __init__(self) -> ButtonState:
        ...


class ButtonStyleSheet:
    '''The appearance of a button() for a given state.'''

    def __init__(
        self,
        proto: Optional[ButtonStyleSheet],
        *,
        shadow_offset: Tuple[_FloatFinite, _FloatFinite] = ...,
        background: Optional[Color] = ...,
        border_radius: _FloatFinite = ...,
        border_width: _FloatFinite = ...,
        border_color: Color = ...,
        text_color: Color = ...,
    ) -> ButtonState:
        ...

    @property
    def shadow_offset(self) -> Tuple[float, float]:
        '''The '(set, copied or defaulted) 'shadow_offset' parameter given to the constructor.'''
    
    @property
    def background(self) -> Optional[Color]:
        '''The '(set, copied or defaulted) 'background' parameter given to the constructor.'''
    
    @property
    def border_radius(self) -> float:
        '''The '(set, copied or defaulted) 'border_radius' parameter given to the constructor.'''
    
    @property
    def border_width(self) -> float:
        '''The '(set, copied or defaulted) 'border_width' parameter given to the constructor.'''
    
    @property
    def border_color(self) -> Color:
        '''The '(set, copied or defaulted) 'border_color' parameter given to the constructor.'''
    
    @property
    def text_color(self) -> Color:
        '''The '(set, copied or defaulted) 'text_color' parameter given to the constructor.'''


ButtonStyle = ButtonStyleSheet


def button(
    state: ButtonState,
    content: Element,
    on_press: Optional[object] = None,
    *,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    min_width: Optional[_U32] = None,
    min_height: Optional[_U32] = None,
    padding: Optional[_U16] = None,
    style: Optional[ButtonStyleSheet] = None,
) -> Element:
    '''A generic widget that produces a message when pressed.'''


###################################################################################################
### Checkbox ######################################################################################
###################################################################################################


class CheckboxStyle:
    '''The appearance of a checkbox() for some state.'''

    def __init__(
        self,
        proto: Optional[Union[CheckboxStyle, str]] = None,
        *,
        background: Color = ...,
        checkmark_color: Color = ...,
        border_radius: _FloatFinite = ...,
        border_width: _FloatFinite = ...,
        border_color: Color = ...,
    ) -> CheckboxStyle:
        ...

    @property
    def background(self) -> Color:
        '''The '(set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def checkmark_color(self) -> Color:
        '''The '(set, copied or defaulted) 'checkmark_color' parameter given to the constructor.'''

    @property
    def border_radius(self) -> float:
        '''The '(set, copied or defaulted) 'border_radius' parameter given to the constructor.'''

    @property
    def border_width(self) -> float:
        '''The '(set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The '(set, copied or defaulted) 'border_color' parameter given to the constructor.'''


class CheckboxStyleSheet:
    '''The appearance of a checkbox().'''

    def __init__(
        self,
        active: CheckboxStyle,
        hovered: Optional[CheckboxStyle] = None,
        active_checked: Optional[CheckboxStyle] = None,
        hovered_checked: Optional[CheckboxStyle] = None,
    ) -> CheckboxStyle:
        ...

    @property
    def active(self) -> CheckboxStyle:
        '''The '(set, copied or defaulted) 'active' parameter given to the constructor.'''

    @property
    def hovered(self) -> CheckboxStyle:
        '''The '(set, copied or defaulted) 'hovered' parameter given to the constructor.'''

    @property
    def active_checked(self) -> CheckboxStyle:
        '''The '(set, copied or defaulted) 'active_checked' parameter given to the constructor.'''

    @property
    def hovered_checked(self) -> CheckboxStyle:
        '''The '(set, copied or defaulted) 'hovered_checked' parameter given to the constructor.'''


def checkbox(
    is_checked: bool,
    label: str,
    f: Optional[Callable[[bool], Optional[object]]] = None,
    *,
    size: Optional[_U16] = None,
    width: Optional[Length] = None,
    spacing: Optional[_U16] = None,
    text_size: Optional[_U16] = None,
    font: Optional[Font] = None,
    style: Optional[CheckboxStyleSheet] = None,
) -> Element:
    '''A box that can be checked.'''


###################################################################################################
### Column ########################################################################################
###################################################################################################


def column(
    children: Iterable[Optional[Element]],
    *,
    spacing: Optional[_U16] = None,
    padding: Optional[_U16] = None,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    max_width: Optional[_U32] = None,
    max_height: Optional[_U32] = None,
    align_items: Optional[Align] = None,
) -> Element:
    '''A container that distributes its contents vertically.'''


###################################################################################################
### Container #####################################################################################
###################################################################################################


class ContainerStyleSheet:
    '''An element decorating some content.'''

    def __init__(
        proto : Optional[ContainerStyleSheet] = None,
        *,
        text_color : Optional[Color] = ...,
        background : Optional[Color] = ...,
        border_radius : _FloatFinite = ...,
        border_width : _FloatFinite = ...,
        border_color : Color = ...,
    ) -> ContainerStyleSheet:
        ...


ContainerStyle = ContainerStyleSheet


def container(
    content: Element,
    *,
    padding: Optional[_U16] = None,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    max_width: Optional[_U32] = None,
    max_height: Optional[_U32] = None,
    align_x: Optional[Length] = None,
    align_y: Optional[Length] = None,
    style: Optional[ContainerStyle] = None,
) -> Element:
    '''An element decorating some content.'''


###################################################################################################
### Image #########################################################################################
###################################################################################################


class ImageHandle:
    '''An image() handle.'''

    @staticmethod
    def from_path(path: Path) -> ImageHandle:
        '''Creates an image handle pointing to the image of the given path.'''

    @staticmethod
    def from_memory(bytes: _BytesLike) -> ImageHandle:
        '''Creates an image handle containing the image data directly.'''


def image(
    handle: ImageHandle,
    *,
    width: Optional[Length] = None,
    heigth: Optional[Length] = None,
) -> Element:
    '''A frame that displays an image while keeping aspect ratio.'''


###################################################################################################
### PickList ######################################################################################
###################################################################################################


class PickListState:
    '''The state of a pick_list().'''

    def __init__(self) -> PickListState:
        ...


class PickListStyle:
    '''The appearance of a pick_list() for some state.'''

    def __init__(
        proto : Optional[Union[PickListStyle, str]] = None,
        *,
        text_color : Color = ...,
        background : Color = ...,
        border_radius : _FloatFinite = ...,
        border_width : _FloatFinite = ...,
        border_color : Color = ...,
        icon_size : _FloatFinite = ...,
    ) -> PickListStyle:
        ...

    @property
    def text_color(self) -> Color:
        '''The '(set, copied or defaulted) 'text_color' parameter given to the constructor.'''

    @property
    def background(self) -> Color:
        '''The '(set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def border_radius(self) -> _FloatFinite:
        '''The '(set, copied or defaulted) 'border_radius' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The '(set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The '(set, copied or defaulted) 'border_color' parameter given to the constructor.'''

    @property
    def icon_size(self) -> _FloatFinite:
        '''The '(set, copied or defaulted) 'icon_size' parameter given to the constructor.'''


class PickListMenu:
    '''The appearance of a pick list menu.'''

    def __init__(
        self,
        proto : Optional[PickListMenu] = ...,
        *,
        text_color : Color = ...,
        background : Color = ...,
        border_width : _FloatFinite = ...,
        border_color : Color = ...,
        selected_text_color : Color = ...,
        selected_background : Color = ...,
    ) -> PickListMenu:
        ...

    @property
    def text_color(self) -> Color:
        '''The '(set, copied or defaulted) 'text_color' parameter given to the constructor.'''

    @property
    def background(self) -> Color:
        '''The '(set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def border_width(self) -> float:
        '''The '(set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The '(set, copied or defaulted) 'border_color' parameter given to the constructor.'''

    @property
    def selected_text_color(self) -> Color:
        '''The '(set, copied or defaulted) 'selected_text_color' parameter given to the constructor.'''

    @property
    def selected_background(self) -> Color:
        '''The '(set, copied or defaulted) 'selected_background' parameter given to the constructor.'''


class PickListStyleSheet:
    '''The appearance of a pick_list().'''

    def __init__(
        self,
        menu : PickListMenu,
        active : PickListStyle,
        hovered : Optional[PickListStyle] = None,
    ) -> PickListStyleSheet:
        ...

    @property
    def menu(self) -> PickListMenu:
        '''The (set, copied or defaulted) 'menu' parameter given to the constructor.'''

    @property
    def active(self) -> PickListStyle:
        '''The (set, copied or defaulted) 'active' parameter given to the constructor.'''

    @property
    def hovered(self) -> PickListStyle:
        '''The (set, copied or defaulted) 'hovered' parameter given to the constructor.'''


def pick_list(
    state : PickListState,
    selected : Optional[str],
    token: object,
    options : Iterable[Optional[str]],
    *,
    text_size : Optional[_U16],
    font : Optional[Font],
    style : Optional[PickListStyleSheet],
) -> Element:
    '''A widget for selecting a single value from a list of options.'''


###################################################################################################
### ProgressBar ###################################################################################
###################################################################################################


# TODO


###################################################################################################
### Radio #########################################################################################
###################################################################################################


# TODO


###################################################################################################
### Row ###########################################################################################
###################################################################################################


def row(
    children: Iterable[Optional[Element]],
    *,
    spacing: Optional[_U16] = None,
    padding: Optional[_U16] = None,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    max_width: Optional[_U32] = None,
    max_height: Optional[_U32] = None,
    align_items: Optional[Align] = None,
) -> Element:
    '''A container that distributes its contents horizontally.'''


###################################################################################################
### Rule ##########################################################################################
###################################################################################################


# TODO


###################################################################################################
### Scrollable ####################################################################################
###################################################################################################


# TODO


###################################################################################################
### Slider ########################################################################################
###################################################################################################


# TODO


###################################################################################################
### Space #########################################################################################
###################################################################################################


# TODO


###################################################################################################
### Svg ###########################################################################################
###################################################################################################


# TODO


###################################################################################################
### TextInput #####################################################################################
###################################################################################################


# TODO


###################################################################################################
### Text ##########################################################################################
###################################################################################################


# TODO


###################################################################################################
### Tooltip #######################################################################################
###################################################################################################


# TODO
