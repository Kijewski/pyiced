from datetime import timedelta
from functools import wraps
from math import isnan, isinf, isfinite
from pathlib import Path
from typing import (
    Annotated, Any, Awaitable, Callable, Iterable, Literal, NoReturn, Optional, Protocol, Tuple,
    TypeVar, Union, final, get_args, get_origin, get_type_hints, overload,
)


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


class _Positive(_CheckableType):
    def check(self, name, value):
        if value <= 0:
            raise ValueError(
                f'Parameter {name:r} must be positive, '
                f'so the value {value:r} is invalid.'
            )


class _NonNegative(_CheckableType):
    def check(self, name, value):
        if value < 0:
            raise ValueError(
                f'Parameter {name:r} must not be negative, '
                f'so the value {value:r} is invalid.'
            )


def _check_annotations(func):
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

DeltaSeconds = TypeVar('DeltaSeconds', Annotated[float, _Finite(), _Positive()])


###################################################################################################
### Wrapped #######################################################################################
###################################################################################################


@final
class Color:
    '''A color in the sRGB color space.'''

    def __init__(
        self,
        r: _FloatNonNan,
        g: _FloatNonNan,
        b: _FloatNonNan,
        a: _FloatNonNan = 1.0,
    ) -> Color:
        ...

    @property
    def r(self) -> _FloatFinite:
        '''Red component, 0.0 – 1.0'''

    @property
    def g(self) -> _FloatFinite:
        '''Green component, 0.0 – 1.0'''

    @property
    def b(self) -> _FloatFinite:
        '''Blue component, 0.0 – 1.0'''

    @property
    def a(self) -> _FloatFinite:
        '''Alpha channel, 0.0 – 1.0 (0.0 = transparent; 1.0 = opaque)'''

    BLACK: Color
    '''Color(0, 0, 0)'''

    WHITE: Color
    '''Color(1, 1, 1)'''

    TRANSPARENT: Color
    '''Color(0, 0, 0, a=0)'''


@final
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


@final
class Align:
    '''Alignment on an axis of a container.'''

    START: Align
    '''Align at the start of the axis.'''

    CENTER: Align
    '''Align at the center of the axis.'''

    END: Align
    '''Align at the end of the axis.'''


@final
class Font:
    '''A font.'''

    def __init__(self, name: str, data: _BytesLike) -> Font:
        ...

    @property
    def name(self) -> Optional[str]:
        '''The (set, copied or defaulted) 'name' parameter given to the constructor.'''


@final
class Point:
    '''A 2D point.'''

    def __init__(
        self,
        x: _FloatFinite,
        y: _FloatFinite,
    ) -> Point:
        ...

    @property
    def x(self) -> _FloatFinite:
        '''The "x" parameter given when constructing this point.'''

    @property
    def y(self) -> _FloatFinite:
        '''The "y" parameter given when constructing this point.'''

    ORIGIN: Point
    '''The origin (i.e. a Point at (0, 0)).'''

    def distance(
        self,
        to: Point,
    ) -> Annotated[_FloatFinite, _NonNegative()]:
        '''Computes the distance to another point.'''


@final
class Size:
    '''An amount of space in 2 dimensions.'''

    def __init__(
        self,
        width: Annotated[float, _NonNan(), _NonNegative()],
        height: Annotated[float, _NonNan(), _NonNegative()],
    ) -> Size:
        ...

    @property
    def width(self) -> Annotated[float, _NonNan(), _NonNegative()]:
        '''The "width" parameter given when constructing this size.'''

    @property
    def height(self) -> Annotated[float, _NonNan(), _NonNegative()]:
        '''The "height" parameter given when constructing this size.'''

    ZERO: Size
    '''A Size with zero width and height.'''

    UNIT: Size
    '''A Size with a width and height of 1 unit.'''

    INFINITY: Size
    '''A Size with infinite width and height.'''

    def pad(
        self,
        pad: Annotated[_FloatNonNan, _NonNegative()],
    ) -> Size:
        '''Increments the Size to account for the given padding.'''


@final
class Rectangle:
    '''A rectangle.'''

    def __init__(
        self,
        top_left: Point,
        size: Size,
    ) -> Rectangle:
        ...

    @staticmethod
    def with_size(
        size: Size,
    ) -> Rectangle:
        '''Creates a new Rectangle with its top-left corner at the origin and with the provided Size.'''

    @property
    def x(self) -> _FloatFinite:
        '''X coordinate of the top-left corner.'''

    @property
    def y(self) -> _FloatFinite:
        '''Y coordinate of the top-left corner.'''

    @property
    def width(self) -> _FloatNonNan:
        '''The "size.width" parameter given when constructing this point.'''

    @property
    def height(self) -> _FloatNonNan:
        '''The "size.height" parameter given when constructing this point.'''

    @property
    def top_left(self) -> Point:
        '''The "top_left" parameter given when constructing this point.'''

    @property
    def size(self) -> Size:
        '''The "size" parameter given when constructing this point.'''


@final
class HorizontalAlignment:
    '''The horizontal alignment of some resource.'''

    LEFT: HorizontalAlignment
    '''Align left'''

    CENTER: HorizontalAlignment
    '''Horizontally centered'''

    RIGHT: HorizontalAlignment
    '''Align right'''


@final
class VerticalAlignment:
    '''The vertical alignment of some resource.'''

    TOP: VerticalAlignment
    '''Align top'''

    CENTER: VerticalAlignment
    '''Vertically centered'''

    BOTTOM: VerticalAlignment
    '''Align bottom'''


@final
class Element:
    '''A displayable widget that can be used in view().'''


Command = Union[Awaitable[Optional[object]], object]
Commands = Iterable[Optional[Command]]


@final
class Subscription:
    '''TODO'''

    NONE: Subscription
    '''TODO'''

    UNCAPTURED: Subscription
    '''TODO'''


def every(
    duration: Union[DeltaSeconds, timedelta],
    token: object,
) -> Subscription:
    '''A Subscription that produces messages at a set interval.'''


@final
class Message:
    '''A message generated through user interaction.'''

    @property
    def kind(self) -> Literal[None, 'mouse', 'window', 'touch', 'keyboard']:
        '''The kind of the native message.'''

    # keyboard

    @property
    def keyboard(self) -> Literal[None, 'keypressed', 'keyreleased', 'characterreceived', 'modifierschanged']:
        '''The kind of the keyboard interaction.'''

    @property
    def key_code(self) -> Optional[str]:
        '''The name of the pressed or released key.'''

    @property
    def shift(self) -> Optional[bool]:
        '''The shift key was pressed / released.'''

    @property
    def alt(self) -> Optional[bool]:
        '''The alt key was pressed / released.'''

    @property
    def logo(self) -> Optional[bool]:
        '''The "logo" key was pressed / released.'''

    @property
    def control(self) -> Optional[bool]:
        '''The control key was pressed / released.'''

    @property
    def characterreceived(self) -> Optional[str]:
        '''The control key was pressed / released.'''

    # mouse

    @property
    def mouse(self) -> Literal[None, 'cursorentered', 'cursorleft', 'cursormoved', 'buttonpressed', 'buttonreleased', 'wheelscrolled']:
        '''A mouse event.'''

    @property
    def button(self) -> Union[None, Literal['left', 'right', 'middle'], _U32]:
        '''The mouse cursor was moved.'''

    @property
    def wheelscrolled(self) -> Literal[None, 'lines', 'pixels']:
        '''The unit of the scroll movement.'''

    @property
    def amount(self) -> Optional[Tuple[float, float]]:
        '''The scroll movement.'''

    # touch

    @property
    def touch(self) -> Literal[None, 'fingerpressed', 'fingermoved', 'fingerlifted', 'fingerlost']:
        '''A touch interaction.'''

    @property
    def finger(self) -> Optional[_U64]:
        '''A unique identifier representing a finger on a touch interaction.'''

    @property
    def position(self) -> Optional[Tuple[float, float]]:
        '''A 2D point for the touch interaction.'''

    # window

    @property
    def window(self) -> Literal[None, 'resized', 'closerequested', 'focused', 'unfocused', 'filehovered', 'filedropped', 'fileshoveredleft']:
        '''The kind of the window message.'''

    @property
    def resized(self) -> Optional[Tuple[int, int]]:
        '''The width and height in pixels or null, if it's not a resize action.'''

    @property
    def file(self) -> Optional[Path]:
        '''The path of the hovering or dropped file.'''


@final
class Clipboard:
    '''A buffer for short-term storage and transfer within and between applications.'''

    def read(self) -> Optional[str]:
        '''Reads the current content of the clipboard as text.'''

    def write(self, value: str) -> None:
        '''Writes the given text contents to the clipboard.'''


class WindowSettings(Protocol):
    '''TODO'''

    size: Tuple[int, int]
    '''Dimensions of the newly crated window.'''

    min_size: Optional[Tuple[int, int]]
    '''The minimum size of the window.'''

    max_size: Optional[Tuple[int, int]]
    '''The maximum size of the window.'''

    resizable: bool
    '''Whether the window should be resizable or not.'''

    decorations: bool
    '''Whether the window should have a border, a title bar, etc. or not.'''

    transparent: bool
    '''Whether the window should be transparent.'''

    always_on_top: bool
    '''Whether the window will always be on top of other windows.'''


class Settings(Protocol):
    '''TODO'''

    default_text_size: int
    '''The text size that will be used by default.'''

    exit_on_close_request: bool
    '''Whether the IcedApp should exit when the user requests the window to close (e.g. the user presses the close button).'''

    antialiasing: bool
    '''If set to true, the renderer will try to perform antialiasing for some primitives.'''

    window: Optional[WindowSettings]
    '''The window settings.'''

    default_font: Optional[Font]
    '''The font that will be used by default.'''


class IcedApp:
    '''TODO'''

    def run(
        self,
        *,
        run: Optional[Callable[[Awaitable[Any]], Union[None, Any, NoReturn]]] = None,
    ) -> NoReturn:
        '''Runs the application.'''

    @property
    def settings(self) -> Optional[Settings]:
        '''The initial settings of the program.'''

    def new(self) -> Optional[Commands]:
        '''Initialize the application.'''

    def title(self) -> str:
        '''The current title of the application.'''

    def should_exit(self) -> bool:
        '''Returns whether the application should be terminated.'''

    def scale_factor(self) -> Annotated[float, _Finite(), _Positive()]:
        '''The scale factor of the application.'''

    def fullscreen(self) -> bool:
        '''True if the program should run in fullscreen mode.'''

    def update(self, msg: Union[Message, object], clipboard: Clipboard) -> Optional[Commands]:
        '''Handles a message and updates the state of the application.'''

    def subscriptions(self) -> Optional[Iterable[Optional[Subscription]]]:
        '''Subscriptions for the current state of the application.'''

    def background_color(self) -> Optional[Color]:
        '''Background color of the application.'''

    def view(self) -> Element:
        '''Element to display in the application.'''


###################################################################################################
### Button ########################################################################################
###################################################################################################


@final
class ButtonState:
    '''The state of a button().'''

    def __init__(self) -> ButtonState:
        ...


@final
class ButtonStyle:
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
    def shadow_offset(self) -> Tuple[_FloatFinite, _FloatFinite]:
        '''The (set, copied or defaulted) 'shadow_offset' parameter given to the constructor.'''
    
    @property
    def background(self) -> Optional[Color]:
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''
    
    @property
    def border_radius(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_radius' parameter given to the constructor.'''
    
    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''
    
    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''
    
    @property
    def text_color(self) -> Color:
        '''The (set, copied or defaulted) 'text_color' parameter given to the constructor.'''


@final
class ButtonStyleSheet:
    '''The appearance of a button()'''

    def __init__(
        self,
        active: ButtonStyle = None,
        hovered: Optional[ButtonStyle] = None,
        pressed: Optional[ButtonStyle] = None,
        disabled: Optional[ButtonStyle] = None,
    ) -> ButtonStyleSheet:
        ...

    @property
    def active(self) -> ButtonStyle:
        '''The (set, copied or defaulted) 'active' parameter given to the constructor.'''

    @property
    def hovered(self) -> ButtonStyle:
        '''The (set, copied or defaulted) 'hovered' parameter given to the constructor.'''

    @property
    def pressed(self) -> ButtonStyle:
        '''The (set, copied or defaulted) 'pressed' parameter given to the constructor.'''

    @property
    def disabled(self) -> ButtonStyle:
        '''The (set, copied or defaulted) 'disabled' parameter given to the constructor.'''


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


@final
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
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def checkmark_color(self) -> Color:
        '''The (set, copied or defaulted) 'checkmark_color' parameter given to the constructor.'''

    @property
    def border_radius(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_radius' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''


@final
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
        '''The (set, copied or defaulted) 'active' parameter given to the constructor.'''

    @property
    def hovered(self) -> CheckboxStyle:
        '''The (set, copied or defaulted) 'hovered' parameter given to the constructor.'''

    @property
    def active_checked(self) -> CheckboxStyle:
        '''The (set, copied or defaulted) 'active_checked' parameter given to the constructor.'''

    @property
    def hovered_checked(self) -> CheckboxStyle:
        '''The (set, copied or defaulted) 'hovered_checked' parameter given to the constructor.'''


def checkbox(
    token: object,
    is_checked: bool,
    label: str,
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


@final
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
    align_x: Optional[Align] = None,
    align_y: Optional[Align] = None,
    style: Optional[ContainerStyle] = None,
) -> Element:
    '''An element decorating some content.'''


###################################################################################################
### Image #########################################################################################
###################################################################################################


@final
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
### no_element ####################################################################################
###################################################################################################


def no_element() -> Element:
    '''A space() with minimum width and height.'''


###################################################################################################
### PickList ######################################################################################
###################################################################################################


@final
class PickListState:
    '''The state of a pick_list().'''

    def __init__(self) -> PickListState:
        ...


@final
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
        '''The (set, copied or defaulted) 'text_color' parameter given to the constructor.'''

    @property
    def background(self) -> Color:
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def border_radius(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_radius' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''

    @property
    def icon_size(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'icon_size' parameter given to the constructor.'''


@final
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
        '''The (set, copied or defaulted) 'text_color' parameter given to the constructor.'''

    @property
    def background(self) -> Color:
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''

    @property
    def selected_text_color(self) -> Color:
        '''The (set, copied or defaulted) 'selected_text_color' parameter given to the constructor.'''

    @property
    def selected_background(self) -> Color:
        '''The (set, copied or defaulted) 'selected_background' parameter given to the constructor.'''


@final
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
    token: object,
    state : PickListState,
    selected : Optional[str],
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


@final
class ProgressBarStyleSheet:
    '''The appearance of a progress_bar()'''

    def __init__(
        self,
        background: Color,
        bar: Color,
        border_radius: _FloatFinite,
    ) -> ProgressBarStyleSheet:
        ...

    @property
    def background(self) -> Color:
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def bar(self) -> Color:
        '''The (set, copied or defaulted) 'bar' parameter given to the constructor.'''

    @property
    def border_radius(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_radius' parameter given to the constructor.'''


ProgressBarStyle = ProgressBarStyleSheet


def progress_bar(
    start: _FloatFinite,
    end: _FloatFinite,
    value: _FloatFinite,
    *,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    style: Optional[ProgressBarStyleSheet] = None,
) -> Element:
    '''A bar that displays progress.'''


###################################################################################################
### Radio #########################################################################################
###################################################################################################


@final
class RadioStyle:
    '''The appearance of a radio() for some state.'''

    def __init__(
        self,
        proto: Optional[Union[RadioStyle, str]] = None,
        *,
        background: Color = ...,
        dot_color: Color = ...,
        border_width: _FloatFinite = ...,
        border_color: Color = ...,
    ) -> RadioStyle:
        ...

    @property
    def background(self) -> Color:
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def dot_color(self) -> Color:
        '''The (set, copied or defaulted) 'dot_color' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''


@final
class RadioStyleSheet:
    '''The appearance of a radio().'''

    def __init__(
        self,
        active: RadioStyle,
        hovered: Optional[RadioStyle] = None,
    ) -> RadioStyleSheet:
        ...

    @property
    def active(self) -> RadioStyle:
        '''The (set, copied or defaulted) 'active' parameter given to the constructor.'''

    @property
    def hovered(self) -> RadioStyle:
        '''The (set, copied or defaulted) 'hovered' parameter given to the constructor.'''


def radio(
    token: object,
    selected: Optional[int],
    value: int,
    label: str,
    *,
    size: Optional[int] = None,
    width: Optional[Length] = None,
    spacing: Optional[int] = None,
    text_size: Optional[int] = None,
    style: Optional[RadioStyleSheet] = None,
) -> Element:
    '''A circular button representing a choice.'''


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


@final
class FillMode:
    '''The fill mode of a rule().'''

    FULL: FillMode
    '''Fill the whole length of the container.'''

    @staticmethod
    def percent(percentage: _FloatNonNan) -> FillMode:
        '''Fill a percent of the length of the container. The rule will be centered in that container.'''

    @staticmethod
    def padded(i: _U16) -> FillMode:
        '''Uniform offset from each end.'''

    def asymmetric_padding(first_pad: _U16, second_pad: _U16) -> FillMode:
        '''Different offset on each end of the rule.'''


@final
class RuleStyleSheet:
    '''The appearance of a rule().'''

    def __init__(
        self,
        proto: Optional[RuleStyleSheet] = None,
        *,
        color: Color = ...,
        width: _U16 = ...,
        radius: _FloatFinite = ...,
        fill_mode: FillMode = ...,
    ) -> RuleStyleSheet:
        ...


RuleStyle = RuleStyleSheet


@overload
def rule(
    *,
    horizontal: Annotated[_U16, _Positive()],
    style: Optional[RuleStyleSheet] = None,
) -> Element:
    '''Display a horizontal or vertical rule for dividing content.'''

@overload
def rule(
    *,
    vertical: Annotated[_U16, _Positive()],
    style: Optional[RuleStyleSheet] = None,
) -> Element:
    ...


###################################################################################################
### Scrollable ####################################################################################
###################################################################################################


@final
class ScrollableState:
    '''The state of a scrollable().'''

    def __init__(self) -> ScrollableState:
        ...

    def scroll(
        self,
        delta_y: _FloatNonNan,
        bounds: Rectangle,
        content_bounds: Rectangle,
    ) -> None:
        '''Apply a scrolling offset to the current ScrollableState, given the bounds of the Scrollable and its contents.'''

    def scroll_to(
        self,
        percentage: _FloatNonNan,
        bounds: Rectangle,
        content_bounds: Rectangle,
    ) -> None:
        '''Moves the scroll position to a relative amount, given the bounds of the Scrollable and its contents.'''

    def offset(
        self,
        bounds: Rectangle,
        content_bounds: Rectangle,
    ) -> _U32:
        '''The current scrolling offset of the ScrollableState, given the bounds of the Scrollable and its contents.'''


@final
class ScrollerStyle:
    '''The appearance of the scroller of a scrollable().'''

    def __init__(
        self,
        proto: Optional[Union[ScrollerStyle, str]],
        *,
        color: Color = ...,
        border_radius: _FloatFinite = ...,
        border_width: _FloatFinite = ...,
        border_color: Color = ...,
    ) -> ScrollerStyle:
        ...

    @property
    def color(self) -> Color:
        '''The (set, copied or defaulted) 'color' parameter given to the constructor.'''

    @property
    def border_radius(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_radius' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''


@final
class ScrollbarStyle:
    '''The appearance a specific state of a scrollable()'''

    def __init__(
        self,
        proto: Optional[Union[ScrollbarStyle, str]] = ...,
        *,
        background: Optional[Color] = ...,
        border_radius: _FloatFinite = ...,
        border_width: _FloatFinite = ...,
        border_color: Color = ...,
        scroller: ScrollerStyle = ...,
    ) -> ScrollbarStyle:
        ...

    @property
    def background(self) -> Optional[Color]:
        '''The (set, copied or defaulted) 'background' parameter given to the constructor.'''

    @property
    def border_radius(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_radius' parameter given to the constructor.'''

    @property
    def border_width(self) -> _FloatFinite:
        '''The (set, copied or defaulted) 'border_width' parameter given to the constructor.'''

    @property
    def border_color(self) -> Color:
        '''The (set, copied or defaulted) 'border_color' parameter given to the constructor.'''

    @property
    def scroller(self) -> ScrollerStyle:
        '''The (set, copied or defaulted) 'scroller' parameter given to the constructor.'''



@final
class ScrollableStyleSheet:
    '''The appearance of a scrollable().'''

    def __init__(
        self,
        active: ScrollbarStyle,
        hovered: Optional[ScrollbarStyle] = None,
        dragging: Optional[ScrollbarStyle] = None,
    ) -> ScrollableStyleSheet:
        ...

    @property
    def active(self) -> ScrollbarStyle:
        '''The (set, copied or defaulted) 'active' parameter given to the constructor.'''

    @property
    def hovered(self) -> ScrollbarStyle:
        '''The (set, copied or defaulted) 'hovered' parameter given to the constructor.'''

    @property
    def dragging(self) -> ScrollbarStyle:
        '''The (set, copied or defaulted) 'dragging' parameter given to the constructor.'''


def scrollable(
    state: ScrollableState,
    children: Iterable[Optional[Element]],
    *,
    spacing: Optional[int] = None,
    padding: Optional[int] = None,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    max_width: Optional[int] = None,
    max_height: Optional[int] = None,
    align_items: Optional[Align] = None,
    scrollbar_width: Optional[int] = None,
    scrollbar_margin: Optional[int] = None,
    scroller_width: Optional[int] = None,
    style: Optional[ScrollableStyleSheet] = None,
) -> Element:
    '''A widget that can vertically display an infinite amount of content with a scrollbar.'''



###################################################################################################
### Slider ########################################################################################
###################################################################################################


# TODO


###################################################################################################
### Space #########################################################################################
###################################################################################################


@final
def space(
    *,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
) -> Element:
    '''An amount of empty space.'''


###################################################################################################
### Svg ###########################################################################################
###################################################################################################


@final
class SvgHandle:
    '''Creates an SVG Handle pointing to the vector image of the given path.'''

    @staticmethod
    def from_path(path: Path) -> SvgHandle:
        '''Creates an SVG handle pointing to the image of the given path.'''

    @staticmethod
    def from_memory(bytes: _BytesLike) -> SvgHandle:
        '''Creates an SVG handle containing the image data directly.'''


def svg(
    handle: SvgHandle,
    *,
    width: Optional[Length] = None,
    heigth: Optional[Length] = None,
) -> Element:
    '''A vector graphics image.'''


###################################################################################################
### Text ##########################################################################################
###################################################################################################


def text(
    label: str,
    *,
    size: Optional[_U16] = None,
    color: Optional[Color] = None,
    font: Optional[Font] = None,
    width: Optional[Length] = None,
    height: Optional[Length] = None,
    horizontal_alignment: Optional[HorizontalAlignment] = None,
    vertical_alignment: Optional[VerticalAlignment] = None,
) -> Element:
    '''A paragraph of text.'''


###################################################################################################
### TextInput #####################################################################################
###################################################################################################


# TODO


###################################################################################################
### Tooltip #######################################################################################
###################################################################################################


@final
class TooltipPosition:
    '''The position of the tooltip.'''

    FOLLOW_CURSOR: TooltipPosition
    '''The tooltip will follow the cursor.'''

    TOP: TooltipPosition
    '''The tooltip will appear on the top of the widget.'''

    BOTTOM: TooltipPosition
    '''The tooltip will appear on the bottom of the widget.'''

    LEFT: TooltipPosition
    '''The tooltip will appear on the left of the widget.'''

    RIGHT: TooltipPosition
    '''The tooltip will appear on the right of the widget.'''


def tooltip(
    content: Element = None,
    tooltip: str = None,
    position: TooltipPosition = None,
    *,
    font: Optional[Font] = None,
    size: Optional[_U16] = None,
    gap: Optional[_U16] = None,
    padding: Optional[_U16] = None,
    style: Optional[ContainerStyleSheet] = None,
) -> Element:
    '''Make a tooltip.'''
