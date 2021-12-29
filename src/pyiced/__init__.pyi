from typing import Any, Awaitable, Callable, Iterable, NoReturn, Optional, Protocol, Tuple, Union

from pyiced._pyiced import *
from pyiced._pyiced import Commands, FloatPositive


__all__ = [
    # states
    'ButtonState', 'PickListState', 'ScrollableState', 'SliderState', 'TextInputState',

    # widgets
    'Element', 'no_element', 'button', 'checkbox', 'column', 'container', 'image', 'pick_list',
    'progress_bar', 'radio', 'row', 'rule', 'scrollable', 'slider', 'space', 'svg', 'text',
    'text_input', 'tooltip',

    # wrapped
    'Align', 'Clipboard', 'Color', 'FillMode', 'Font', 'HorizontalAlignment', 'ImageHandle',
    'Instant', 'Length', 'Line', 'Message', 'Point', 'Rectangle', 'SliderHandle',
    'SliderHandleShape', 'SvgHandle', 'TextInputCursor', 'TooltipPosition', 'VerticalAlignment',

    # styles
    'ButtonStyle', 'ButtonStyleSheet', 'CheckboxStyle', 'CheckboxStyleSheet', 'ContainerStyleSheet',
    'PaneGridStyleSheet', 'PickListMenu', 'PickListStyle', 'PickListStyleSheet',
    'ProgressBarStyleSheet', 'RadioStyle', 'RadioStyleSheet', 'RuleStyleSheet',
    'ScrollableStyleSheet', 'ScrollbarStyle', 'ScrollerStyle', 'Size', 'SliderStyle',
    'SliderStyleSheet', 'TextInputStyle', 'TextInputStyleSheet',

    # subscription
    'every', 'stream', 'Subscription',

    # interfaces
    'IcedApp', 'Settings', 'WindowSettings',

    # aliases
    'ButtonStyle', 'ContainerStyle', 'PaneGridStyle', 'ProgressBarStyle',
]


class WindowSettings(Protocol):
    '''(Immutable) settings of the IcedApp window.'''

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
    '''(Immutable) settings of the IcedApp application.'''

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
    '''An interactive application.'''

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

    def scale_factor(self) -> FloatPositive:
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
