# Copyright (c) 2021 René Kijewski <pypi.org@k6i.de>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

from abc import ABCMeta, abstractmethod
from asyncio import get_event_loop, Queue as AsyncQueue, run as _run, run_coroutine_threadsafe
from os.path import abspath, dirname, join
from queue import Queue as SyncQueue
from threading import Thread
from typing import Any, Awaitable, Callable, Iterable, NoReturn, Optional, Tuple, Union

from pyiced import _pyiced
from pyiced._pyiced import (
    # states
    ButtonState, PickListState, ScrollableState, SliderState, TextInputState,

    # widgets
    Element, no_element, button, checkbox, column, container, image, pick_list,
    progress_bar, radio, row, rule, scrollable, slider, space, svg, text,
    text_input, tooltip,

    # wrapped
    Align, Clipboard, Color, FillMode, Font, HorizontalAlignment, Icon, ImageHandle,
    Instant, Length, Line, Message, Point, Rectangle, SliderHandle,
    SliderHandleShape, SvgHandle, TextInputCursor, TooltipPosition, VerticalAlignment,

    # styles
    ButtonStyle, ButtonStyleSheet, CheckboxStyle, CheckboxStyleSheet, ContainerStyleSheet,
    PaneGridStyleSheet, PickListMenu, PickListStyle, PickListStyleSheet,
    ProgressBarStyleSheet, RadioStyle, RadioStyleSheet, RuleStyleSheet,
    ScrollableStyleSheet, ScrollbarStyle, ScrollerStyle, Size, SliderStyle,
    SliderStyleSheet, TextInputStyle, TextInputStyleSheet,

    # subscription
    every, stream, Subscription,
)
from pyiced._pyiced import __author__, __license__, __version__  # noqa


# KEEP SYNCHRONOUS TO MODULE EXPORTS
__all__ = [
    # states
    'ButtonState', 'PickListState', 'ScrollableState', 'SliderState', 'TextInputState',

    # widgets
    'Element', 'no_element', 'button', 'checkbox', 'column', 'container', 'image', 'pick_list',
    'progress_bar', 'radio', 'row', 'rule', 'scrollable', 'slider', 'space', 'svg', 'text',
    'text_input', 'tooltip',

    # wrapped
    'Align', 'Clipboard', 'Color', 'FillMode', 'Font', 'HorizontalAlignment', 'Icon', 'ImageHandle',
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

if hasattr(_pyiced, 'findfont'):
    from pyiced._pyiced import (  # noqa
        FontFamily, FontId, FontStretch, FontStyle, FontWeight, findfont, systemfonts,
    )

    __all__.extend((
        'FontFamily', 'FontId', 'FontStretch', 'FontStyle', 'FontWeight', 'findfont', 'systemfonts',
    ))


Command = Union[Awaitable[Optional[object]], object]
Commands = Iterable[Optional[Command]]

ContainerStyle = ContainerStyleSheet
PaneGridStyle = PaneGridStyleSheet
ProgressBarStyle = ProgressBarStyleSheet
RuleStyle = RuleStyleSheet
TooltipStyleSheet = ContainerStyle
TooltipStyle = TooltipStyleSheet

DefaultIcon = Icon(join(dirname(abspath(__file__)), 'logo.png'))


class WindowSettings:
    '''
    (Immutable) settings of the :class:`~pyiced.IcedApp` window.
    '''

    size: Tuple[int, int] = (1024, 768)
    '''
    Initial dimensions of the newly crated window.
    '''

    min_size: Optional[Tuple[int, int]] = None
    '''
    The minimum size of the window.
    '''

    max_size: Optional[Tuple[int, int]] = None
    '''
    The maximum size of the window.
    '''

    resizable: bool = True
    '''
    Whether the window should be resizable or not.
    '''

    decorations: bool = True
    '''
    Whether the window should have a border, a title bar, etc. or not.
    '''

    transparent: bool = False
    '''
    Whether the window should be transparent.
    '''

    always_on_top: bool = False
    '''
    Whether the window will always be on top of other windows.
    '''

    icon: Optional[Icon] = DefaultIcon
    '''
    TODO
    '''


class Settings:
    '''
    (Immutable) settings of the :class:`~pyiced.IcedApp` application.
    '''

    default_text_size: int = 20
    '''
    The text size that will be used by default.
    '''

    exit_on_close_request: bool = True
    '''
    Whether the :class:`~pyiced.IcedApp` should exit when the user requests the window to close
    (e.g. the user presses the close button).
    '''

    antialiasing: bool = True
    '''
    If set to true, the renderer will try to perform antialiasing for some primitives.

    Enabling it can produce a smoother result in some widgets, like the Canvas, at a performance
    cost.
    '''

    window: WindowSettings = WindowSettings()
    '''
    The window settings.
    '''

    default_font: Font = Font.DEFAULT
    '''
    The font that will be used by default.

    If `None` or `Font.DEFAULT` is provided, a default system font will be chosen.
    '''


class IcedApp(metaclass=ABCMeta):
    '''
    An interactive application.

    An application can execute asynchronous actions by returning :class:`~pyiced.Commands` in some
    of its methods. A debug view can be toggled by pressing F12.
    '''

    def run(
        self, *,
        run: Optional[Callable[[Awaitable[Any]], Union[None, Any, NoReturn]]] = None,
    ) -> NoReturn:
        '''
        Runs the application.

        This method will take control of the current thread and will NOT return unless there is an
        error during startup.

        It should probably be that last thing you call in your main function.

        Arguments
        ---------
        run
            Coroutine executor. Defaults to :func:`asyncio.run()`.
        '''
        return _run_iced(self, run=run)

    settings: Settings = Settings()
    '''
    The initial settings of the program.

    Only queried once.
    '''

    def new(self) -> Optional[Commands]:
        '''
        Initialize the application.

        You can return :class:`~pyiced.Commands` if you need to perform some async action in the
        background on startup. This is useful if you want to load state from a file, perform an
        initial HTTP request, etc.
        '''
        return None

    def title(self) -> str:
        '''
        The current title of the application.

        This title can be dynamic! The runtime will automatically update the title of your
        application when necessary.
        '''
        return f'PyIced {__version__}'

    def should_exit(self) -> bool:
        '''
        Returns whether the application should be terminated.

        This will kill the Python instance, too.
        '''
        return False

    def scale_factor(self) -> float:
        '''
        Returns the scale factor of the application.

        It can be used to dynamically control the size of the UI at runtime (i.e. zooming).

        For instance, a scale factor of 2.0 will make widgets twice as big, while a scale factor of
        0.5 will shrink them to half their size.
        '''
        return 1.0

    def fullscreen(self) -> bool:
        '''
        True if the program should run in fullscreen mode.

        The runtime will automatically transition your application if a new mode is returned.
        '''
        return False

    def update(self, msg: Union[Message, object], clipboard: Clipboard) -> Optional[Commands]:
        '''
        Handles a message and updates the state of the application.

        This is where you define your update logic. All the messages, produced by either user
        interactions or commands, will be handled by this method.
        The method call be must executed quite fast. Long running tasks should be executed
        asynchronously.

        Any :class:`~pyiced.Command` returned will be executed immediately in the background.

        Arguments
        ---------
        msg: Union[Message | object]
            A message to handle. Generated either through user iteraction, or though an
            (asynchronous) :class:`pyiced.Command`.
        clipboard: Clipboard
            The OS's inter-application message buffer. Can only be interacted with during this call
            to :meth:`~Pyiced.IcedApp.update()`. Accessing it later or in another thread may crash
            the application.

        Returns
        -------
        Optional[Commands]
            The update invocation may return a list of coroutines for asynchronous message handling,
            e.g. to open a socket.
        '''
        return None

    def subscriptions(self) -> Optional[Iterable[Optional[Subscription]]]:
        '''
        Returns the event :ref:`subscriptions <subscriptions:Event Listening>` for the current state
        of the application.

        A subscription will be kept alive as long as you keep returning it, and the messages
        produced will be handled by update.
        '''
        return None

    def background_color(self) -> Optional[Color]:
        '''
        Returns the background color of the application. Defaults to white.
        '''
        return Color.WHITE

    @abstractmethod
    def view(self) -> Element:
        '''
        Returns the :ref:`widget <elements:Displayable Elements>` to display in the application.

        These widgets can produce messages based on user interaction.
        '''
        raise NotImplementedError('You need to implement (at least) IcedApp.view().')


def _run_iced(app: IcedApp, *, run=None) -> NoReturn:
    return _pyiced.run_iced(
        new=app.new,
        title=app.title,
        update=app.update,
        should_exit=app.should_exit,
        scale_factor=app.scale_factor,
        fullscreen=app.fullscreen,
        view=app.view,
        subscriptions=app.subscriptions,
        settings=app.settings,
        background_color=app.background_color,
        taskmanager=_make_loop(run),
    )


def _make_loop(run=None):
    put_task = SyncQueue(1)
    thread = Thread(
        None,
        run if run is not None else _run,
        args=(_thread_code(put_task),),
        name='PyIced-AsyncLoop',
    )
    thread.daemon = True
    thread.start()
    return put_task.get()


async def _thread_code(put_task):
    loop = get_event_loop()
    task_queue = AsyncQueue()
    put_task.put((
        loop,
        lambda task=None: run_coroutine_threadsafe(task_queue.put(task), loop),
    ))
    tasks = set()
    while True:
        taskobj = await task_queue.get()
        if taskobj is None:
            break

        task = loop.create_task(_process_task(taskobj))
        tasks.add(task)
        task.add_done_callback(tasks.remove)


async def _process_task(taskobj):
    value = err = None
    try:
        value = await taskobj.task
    except BaseException as ex:
        err = ex

    try:
        taskobj.result = err, value
        taskobj()
    finally:
        if isinstance(err, SystemExit):
            raise err
